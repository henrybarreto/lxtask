/* $Id: functions.h 3940 2008-02-10 22:48:45Z nebulon $
 *
 * Copyright (c) 2006 Johannes Zellner, <webmaster@nebulon.de>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Library General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA 02111-1307, USA.
 */

#ifndef FUNCTIONS_H
#define FUNCTIONS_H

#include <gtk/gtk.h>
#include <glib.h>
#include <dirent.h>
#include <pwd.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <sys/param.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "types.h"
#include "interface.h"

#ifdef __LINUX__
#include "xfce-taskmanager-linux.h"
#endif

#define PROC_DIR_1 "/compat/linux/proc"
#define PROC_DIR_2 "/emul/linux/proc"
#define PROC_DIR_3 "/proc"

struct GPUInfo {
    gdouble gpu_usage;
    gint gpu_mem_total;
    gint gpu_mem_used;
};

gboolean refresh_task_list(void);
gdouble get_cpu_usage(system_status *sys_stat);
struct GPUInfo get_gpu_info();

/* Configurationfile support */
void load_config(void);
void save_config(void);

#endif


