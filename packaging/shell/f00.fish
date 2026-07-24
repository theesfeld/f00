# f00tils — fish: default coreutils replacement via PATH
# See /etc/profile.d/f00.sh for the same policy.

set -l _f00_libbin /usr/lib/f00/bin
set -l _f00_cfg
if set -q XDG_CONFIG_HOME
    set _f00_cfg $XDG_CONFIG_HOME/f00/config
else
    set _f00_cfg $HOME/.config/f00/config
end

set -l _f00_on 1
if test -f $_f00_cfg
    if string match -qir '^\s*replace\s*=\s*(false|no|0|none)(\s|#|$)' (cat $_f00_cfg 2>/dev/null)
        set _f00_on 0
    end
end

if test -d $_f00_libbin; and test $_f00_on -eq 1
    if not contains -- $_f00_libbin $PATH
        set -gx PATH $_f00_libbin $PATH
    end
end
