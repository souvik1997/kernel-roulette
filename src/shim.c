#include <linux/init.h>
#include <linux/kernel.h>
#include <linux/module.h>
#include <linux/slab.h>
#include <linux/fs.h>
#include <linux/uaccess.h>

#define SUCCESS 0

/*
 * The entry points for the kernel module in Rust. We define
 * entry points in C for the module_init and module_exit macros.
 */
extern int rust_mod_init(void);
extern int rust_mod_exit(void);

/*
 * A utility function to print a string.
 * NOTE: `str` is not necessarily NULL-terminated.
 */
void puts_c(u64 length, char* str) {
  printk(KERN_DEBUG "%.*s", (int)length, str);
}

/*
 * Define the abort() function. We use the BUG() macro, which generates a ud2 instruction.
 */
void abort(void) {
  BUG();
}

/*
 * Call the panic() function.
 */
void panic_c(void) {
  panic("Panic!");
}

/*
 * Re-exported memory management functions
 */
void* kmalloc_c(size_t size) {
  return kmalloc(size, GFP_KERNEL);
}

void kfree_c(void* ptr) {
  kfree(ptr);
}

void* krealloc_c(const void* p, size_t new_size) {
  return krealloc(p, new_size, GFP_KERNEL);
}

/*
 * This isn't really a nanosecond timer but its close enough.
 */
u64 nanosecond_timer_c(void) {
  return get_jiffies_64();
}

static int rl_device_open(struct inode* inode, struct file* filp);
static int rl_device_release(struct inode* inode, struct file* filp);
static ssize_t rl_device_read(struct file* filp,	/* see include/linux/fs.h   */
                              char __user* buffer,	/* buffer to fill with data */
                              size_t length,	/* length of the buffer  */
                              loff_t* offset /* the file offset */);
static struct file_operations rl_driver_fops = {
  .read = rl_device_read,
  .open = rl_device_open,
  .release = rl_device_release,
};
static int rl_dev_major_num = 0;
static const char rl_device_name[] = "kernel-roulette";
static const size_t MAX_BUFFER_SIZE = 256;

struct rl_state {
  char* data;
  size_t length;
};

/*
 * Functions defined in Rust
 */
extern u8 sample(void);
extern void set_chance(u8 chance);
extern u8 get_chance(void);

static int rl_device_open(struct inode* inode, struct file* filp) {
  u8 sampled;
  struct rl_state* state;

  // Increase module usage count.
  // This prevents unloading the module while the file is open
  try_module_get(THIS_MODULE);

  // Get a sample from Rust
  sampled = sample(); // may panic!

  // Allocate some state to keep track of data
  state = kmalloc_c(sizeof(struct rl_state));

  // Allocate a string of size MAX_BUFFER_SIZE
  state->data = kmalloc_c(MAX_BUFFER_SIZE);

  // Format data and store in string
  snprintf(state->data, MAX_BUFFER_SIZE, "Survived... sampled value is %d, which is >= %d\n", sampled, get_chance());
  // Update length
  state->length = strnlen(state->data, MAX_BUFFER_SIZE);

  // Set private_data to the state we allocated
  filp->private_data = state;
  return SUCCESS;
}

static int rl_device_release(struct inode* inode, struct file* filp) {
  // Decrement usage count so this module can be unloaded
  module_put(THIS_MODULE);

  // Free the string
  kfree_c(((struct rl_state*)filp->private_data)->data);

  // Free the state struct
  kfree_c(filp->private_data);

  return SUCCESS;
}

static ssize_t rl_device_read(struct file* filp, char __user* buffer, size_t length, loff_t* offset) {
  size_t total_length;
  char* data;
  size_t remaining;
  size_t bytes_to_read;

  // Disallow reading past MAX_BUFFER_SIZE
  if (*offset > MAX_BUFFER_SIZE) {
    return 0;
  }

  // The total length of the data
  total_length = ((struct rl_state*)filp->private_data)->length;

  // The data itself
  data = ((struct rl_state*)filp->private_data)->data;

  // How many bytes are remaining to be read?
  remaining = total_length - *offset;

  // How many bytes should we read?
  bytes_to_read = remaining > length ? length : remaining;

  // Copy the data to userspace
  if (copy_to_user(buffer, data + *offset, bytes_to_read) != 0) {
    return -EFAULT;
  }

  // Update the offset
  *offset += bytes_to_read;

  // Return bytes that were read
  return bytes_to_read;
}

/*
 * The entry points in C
 */
static int _mod_init(void) {

  // Register a character device
  rl_dev_major_num = register_chrdev(0 /* allocate a major number */, rl_device_name, &rl_driver_fops);

  if (rl_dev_major_num < 0) {
    printk(KERN_ALERT "failed to register character device: got major number %d\n", rl_dev_major_num);
    return rl_dev_major_num;
  }

  printk(KERN_INFO "Registered %s with major device number %d\n", rl_device_name, rl_dev_major_num);
  printk(KERN_INFO "Run /bin/mknod /dev/%s c %d 0\n", rl_device_name, rl_dev_major_num);

  return rust_mod_init();
}

static void _mod_exit(void) {
  rust_mod_exit();
  unregister_chrdev(rl_dev_major_num, rl_device_name);
}

module_init(_mod_init);
module_exit(_mod_exit);

// TODO: This doesn't properly set the .modinfo section properly on the final .ko file
MODULE_LICENSE("GPL");
