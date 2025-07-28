def main [puzzle: string = 'yin-yang', size: int = 17] {
  let h = http get $'https://www.puzzle-($puzzle).com/?size=($size)'
  $h | rg -o "task.*" | lines
  {}
  | insert task {$h | rg "task.*'(.*)';" -or '$1'}
  | insert width {$h | rg 'puzzleWidth: (\S*),' -or '$1'}
  | insert id {$h | rg 'puzzleID">(.*)</span>' -or '$1' | sd , ''}
  | update id {if $in == '' {$h | split row -r ',|\{'
    | where $it =~ 'specialDate|ident'
    | str trim | split row "'"
    | [$in.1 $in.4] | str join .
  } else {}}
  | do {|i|
    $'($in.width);($in.task)' | save $'($puzzle)/puzzle/($i.id).txt'
  } $in
}
