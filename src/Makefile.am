lxtask_CPPFLAGS = \
	-I$(top_srcdir)/include \
	-DG_LOG_DOMAIN=\"xfce4-taskmanager\" \
	-DPACKAGE_LOCALE_DIR=\"$(localedir)\"

bin_PROGRAMS = lxtask

lxtask_SOURCES = \
	main.c \
	callbacks.c \
	callbacks.h \
	functions.c \
	functions.h \
	interface.c \
	interface.h \
	xfce-taskmanager-linux.c \
	xfce-taskmanager-linux.h \
	types.h \
	utils.c \
	utils.h

lxtask_CFLAGS =	$(GTK_CFLAGS)

external.o: external.rs 
	rustc $< --crate-type staticlib --emit obj -C panic=abort -C overflow-checks=no -o $@

lxtask_LDADD = $(GTK_LIBS) external.o

RUST_LIBSTD_PATH = $(shell rustc --print sysroot)/lib

RUST_LIBSTD_BASENAME = $(notdir $(basename $(shell ls $(RUST_LIBSTD_PATH)/libstd-*.so)))

RUST_LIBSTD_NAME = $(patsubst lib%,%,$(basename $(RUST_LIBSTD_BASENAME)))

lxtask_LDFLAGS = -L$(RUST_LIBSTD_PATH) -l$(RUST_LIBSTD_NAME)
