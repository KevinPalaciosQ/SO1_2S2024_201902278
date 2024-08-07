#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/init.h>
#include <linux/proc_fs.h> 
#include <linux/seq_file.h> 
#include <linux/mm.h> 
#include <linux/sched.h> 
#include <linux/timer.h> 
#include <linux/jiffies.h> 

MODULE_LICENSE("GPL");
MODULE_AUTHOR("Tu Nombre");
MODULE_DESCRIPTION("Modulo para leer informacion de memoria y CPU");
MODULE_VERSION("1.0");
#define PROC_NAME "sysinfo" // nombre del archivo en /proc

static int sysinfo_show(struct seq_file *m, void *v) {
    struct sysinfo si; // estructura que contiene la informacion de la memoria
    struct task_struct *task; // para recorrer los procesos
    struct task_struct *child; // para los procesos hijos
    struct list_head *list; // para recorrer la lista de hijos

    si_meminfo(&si); // obtiene la informacion de la memoria

    seq_printf(m, "Total RAM: %lu KB\n", si.totalram * 4);
    seq_printf(m, "Free RAM: %lu KB\n", si.freeram * 4);
    seq_printf(m, "Shared RAM: %lu KB\n", si.sharedram * 4);
    seq_printf(m, "Buffer RAM: %lu KB\n", si.bufferram * 4);
    seq_printf(m, "Total Swap: %lu KB\n", si.totalswap * 4);
    seq_printf(m, "Free Swap: %lu KB\n", si.freeswap * 4);
    seq_printf(m, "Number of processes: %d\n", num_online_cpus());

    // Iterar sobre todos los procesos
    for_each_process(task) {
        seq_printf(m, "\nPID: %d | Nombre: %s\n", task->pid, task->comm);
        if (task->real_parent) {
            seq_printf(m, "  Padre PID: %d | Nombre: %s\n", task->real_parent->pid, task->real_parent->comm);
        }
        list_for_each(list, &task->children) {
            child = list_entry(list, struct task_struct, sibling);
            seq_printf(m, "  Hijo PID: %d | Nombre: %s\n", child->pid, child->comm);
        }
    }

    return 0;
}

static int sysinfo_open(struct inode *inode, struct file *file) {
    return single_open(file, sysinfo_show, NULL);
}

static const struct proc_ops sysinfo_ops = {
    .proc_open = sysinfo_open,
    .proc_read = seq_read,
    .proc_lseek = seq_lseek,
    .proc_release = single_release,
};

static int __init sysinfo_init(void) {
    proc_create(PROC_NAME, 0, NULL, &sysinfo_ops);
    printk(KERN_INFO "sysinfo module loaded\n");
    return 0;
}

static void __exit sysinfo_exit(void) {
    remove_proc_entry(PROC_NAME, NULL);
    printk(KERN_INFO "sysinfo module unloaded\n");
}

module_init(sysinfo_init);
module_exit(sysinfo_exit);
