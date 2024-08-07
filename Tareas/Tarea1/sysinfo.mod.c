#include <linux/module.h>
#define INCLUDE_VERMAGIC
#include <linux/build-salt.h>
#include <linux/elfnote-lto.h>
#include <linux/export-internal.h>
#include <linux/vermagic.h>
#include <linux/compiler.h>

#ifdef CONFIG_UNWINDER_ORC
#include <asm/orc_header.h>
ORC_HEADER;
#endif

BUILD_SALT;
BUILD_LTO_INFO;

MODULE_INFO(vermagic, VERMAGIC_STRING);
MODULE_INFO(name, KBUILD_MODNAME);

__visible struct module __this_module
__section(".gnu.linkonce.this_module") = {
	.name = KBUILD_MODNAME,
	.init = init_module,
#ifdef CONFIG_MODULE_UNLOAD
	.exit = cleanup_module,
#endif
	.arch = MODULE_ARCH_INIT,
};

#ifdef CONFIG_RETPOLINE
MODULE_INFO(retpoline, "Y");
#endif



static const char ____versions[]
__used __section("__versions") =
	"\x14\x00\x00\x00\xd4\x9e\xe4\x23"
	"single_open\0"
	"\x14\x00\x00\x00\x7c\x24\xc7\x40"
	"si_meminfo\0\0"
	"\x14\x00\x00\x00\x27\xc8\x2d\x2c"
	"seq_printf\0\0"
	"\x1c\x00\x00\x00\x20\x06\x0d\xc6"
	"__num_online_cpus\0\0\0"
	"\x14\x00\x00\x00\xa9\x21\x81\x91"
	"init_task\0\0\0"
	"\x1c\x00\x00\x00\xcb\xf6\xfd\xf0"
	"__stack_chk_fail\0\0\0\0"
	"\x1c\x00\x00\x00\xa7\xd1\x24\xea"
	"remove_proc_entry\0\0\0"
	"\x14\x00\x00\x00\xc7\x49\x49\x4a"
	"seq_read\0\0\0\0"
	"\x14\x00\x00\x00\x85\xf0\xa9\xdc"
	"seq_lseek\0\0\0"
	"\x18\x00\x00\x00\x7c\x74\x2d\xa0"
	"single_release\0\0"
	"\x14\x00\x00\x00\xbb\x6d\xfb\xbd"
	"__fentry__\0\0"
	"\x14\x00\x00\x00\x5c\x68\x0f\x9f"
	"proc_create\0"
	"\x10\x00\x00\x00\x7e\x3a\x2c\x12"
	"_printk\0"
	"\x1c\x00\x00\x00\xca\x39\x82\x5b"
	"__x86_return_thunk\0\0"
	"\x18\x00\x00\x00\x83\xfa\x0c\xf5"
	"module_layout\0\0\0"
	"\x00\x00\x00\x00\x00\x00\x00\x00";

MODULE_INFO(depends, "");


MODULE_INFO(srcversion, "61923592EE3719DE16A1855");
