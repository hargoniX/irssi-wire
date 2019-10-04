#include "glib.h"
#include "glibconfig.h"

#include "irssi/src/common.h"
#include "irssi/src/core/args.h"
#include "irssi/src/core/channels-setup.h"
#include "irssi/src/core/channels.h"
#include "irssi/src/core/chat-protocols.h"
#include "irssi/src/core/commands.h"
#include "irssi/src/core/core.h"
#include "irssi/src/core/expandos.h"
#include "irssi/src/core/ignore.h"
#include "irssi/src/core/iregex.h"
#include "irssi/src/core/levels.h"
#include "irssi/src/core/line-split.h"
#include "irssi/src/core/log.h"
#include "irssi/src/core/masks.h"
#include "irssi/src/core/misc.h"
#include "irssi/src/core/module.h"
#include "irssi/src/core/modules-load.h"
#include "irssi/src/core/modules.h"
#include "irssi/src/core/nicklist.h"
#include "irssi/src/core/nickmatch-cache.h"
#include "irssi/src/core/pidwait.h"
#include "irssi/src/core/queries.h"
#include "irssi/src/core/rawlog.h"
#include "irssi/src/core/recode.h"
#include "irssi/src/core/refstrings.h"
#include "irssi/src/core/session.h"
#include "irssi/src/core/settings.h"
#include "irssi/src/core/signals.h"
#include "irssi/src/core/special-vars.h"
#include "irssi/src/core/window-item-def.h"

#include "recs.h"
