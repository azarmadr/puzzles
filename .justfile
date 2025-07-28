get puzzle='yin-yang' size='15':
  nu etc/get-puzzle.nu {{puzzle}} {{size}}

import 'just/rust.just'
default: \
    (test "check_grid_transforms_for_3x3") \
    (example "simple puzzle/example-30x30.txt") \
    # (fmt) \

dos2unix:
  fd . -tf -X dos2unix

mod tui
mod yin-yang
