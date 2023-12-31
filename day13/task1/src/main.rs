fn main() {
    let field1 = r"
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.
".trim();

    let field2 = r"
#...##..#
#....#..#
..##..###
#####.##.
.........
#####.##.
..##..###
#....#..#
".trim();

    let fields = vec![
r".##.#....####..
###....###...##
.....###..#####
.....###..#####
###....###...##
.##.#....####..
#.#..#.#...#.##
##..#.##.#.##..
.#.#.#.#..#.###
.#.###.#..#..##
.#..#.....#.###
..###.###..##..
...#.#..#..####
#.####.##..#.##
..##.##.###.###
#.#.##.#..#.###
#..###...###.#.",

r"....##..##...
..#..####..#.
.############
.############
..#..####..#.
....##..##...
###......#.##",

r".##..#..#.###
.##.#...#..##
..#.######.#.
.......#.....
.......#.....
..#.######.#.
.##.#...#..##
.##..#..#.###
####.#####.#.
####.#####.#.
.##.....#.###",

r"#..##.##.
#...###..
##..###.#
##.##..##
##.##..##
##..###.#
#...###..
#..##.##.
##.##.###
..##.....
..#...###
..##..#..
..#...#..
..#...###
..##.....",

r"..##.##.##...
##.#......###
..########...
#####..######
..#......#...
.####..####..
#....##....##
....#..#.....
#...####...##
##.#....#.###
#...#..#...##
#####..######
##..#..#..###
.#...##...#..
..#.#..#.#...",

r".####..
.####..
##..###
..##...
.####..
##..###
#.##.#.
.####..
#####.#",

r"#.#..##..#.#..###
#.##.##.##.#.####
#.#......#.#.#...
#..........#.####
.##########..#.##
#..######..####..
##...###..##.####
#..........######
#.########.#..#..
.#..#..#..#.###..
.##..##..##..#...
#.########.#..#..
...#....#...#..##",

r"#......
.#.#.#.
.##..##
.##...#
.#.#.#.
#......
#.###..
#.###..
#......
.#.#.#.
.##...#",

r".##.##.
#.##..#
....##.
#..####
##.#..#
####..#
.#.....
##.#...
..#.##.
###.##.
###.##.",

r"....#.##.
#.#....#.
#.#....#.
....#.###
.#....###
.####.##.
...##....
##.######
.##.#....
#..#.##.#
.####.###
#...####.
#...####.",

r"..##...###..#..#.
..##...###..#..#.
####.#.#.#..#.#.#
..###.#....#.####
##.###.#####...##
...#.#...##..#...
...#..###....#..#
###.....#......#.
#.###...#####.##.
..###.#.##..#..#.
..#####.#.#......
###....#.#.#..##.
..#.####.##.#....",

r".###..#.#
.###..#.#
..##.##.#
#.#####.#
#.#.#####
.###.....
##.#.##.#
.#.###.#.
#.####.##
#.####.##
.#.###.#.
##.#.##.#
.###.#...",

r"########.
...#.#.##
#..#####.
#..#.....
..##.#...
..##...#.
..##...#.
..##.#...
#.##.....
#..#####.
...#.#.##
########.
##.##.###
....#####
..#.....#
..#.....#
....#####",

r"....##....#
.##.#.####.
#..#.######
#..###.##.#
....#.#..#.
#.#########
....##....#
#..##......
#####..##..
#..#..####.
.##...#..#.",

r"........#......
.........#.####
##....##....#..
.##..##...##..#
.##..##.##...#.
########.####.#
########.####.#
.##..##.##...#.
.##..##...##..#
##....##....#..
.........#.####
........#......
.#...##.#.#...#
#.#..#.#....##.
###..####.####.
#......#...##.#
##########...##",

r"##.#.#..#####
##.###.#.#...
.....##..#...
####.######..
.##..###.#.##
#.####.####..
.#...#..#.#..
.###.##..#.##
##.##...#####
#..####.#####
...##.....###
#.#...###..##
#.#...###..##
...##...#.###
#..####.#####",

r"....#..##.#..####
#.##.######.###.#
#####.#.#.#.#....
#####.#.#.#.#....
#.##.######.###.#
....#..####..####
..##.#####.....##
#.#.#....###.#..#
..#...#.##.####..
.##...###.#.###..
.##...###.#.###..",

r".###.#..#.###..
#.#.#.##.#.#.##
..#.######.#...
..#...##...#...
#.##.####..#.##
##.########.###
..#..####..#...
#.##......##.##
##.#..##..#.###",

r"##.#........#.###
###..##..##..####
#..#..#.....#..##
...#.##..##.#....
#..#..#..#..#..##
...###.##.###....
#.#####..#####.##
....########.....
..#.#..##..#.#...
.#.###.##.###.#..
..##..####..##...
#.#..#....#..#.##
.#..##.##.##..#..
####..####..#####
##.#.##..##.#.###",

r"#.#.##.#.###..#
##......###.##.
....##...##....
####..######..#
.##....##.#####
#........#.....
##......####..#",

r"..###.#..####
..###.###..##
..###.###.###
..#.#..#....#
..#.#..##...#
..###.###.###
..###.###..##
..###.#..####
###..#...#.##
###....#.#.#.
...#.###..##.
######.###..#
##......#..#.",

r".#######.##.#####
.#...#........#..
##...###....###..
##...###....###..
.#...#........#..
.#######.##.#####
#.####..####.####
#.##..########..#
.#.#...#....#...#
..###..#....#..##
...#.#..#..#..#.#",

r"##.#...##...#
...#.#....#.#
###.#.####.#.
..###.####.##
##.#.#....#.#
##..#..##..#.
......####...
##.#.#....#.#
..#.##....##.
......#..#...
...##.####.##
..#....##....
##...######.#",

r"............#.#
##.#....#.###..
.##.#..#.##...#
###########..##
....####....##.
#..##..##..##..
####....####.#.
...#.##.#......
#..#....#..###.
.#........#.###
.#.#....#.#...#
.##......##.###
....####....#..
##.#....#.##.##
#..######..###.
#...#..#...###.
#...#..#...###.",

r".#..#.#.#
.#..#.#.#
....##..#
#.#######
##..#..#.
.#.#.#..#
...#....#
.###..#.#
...#..###
.#.##.##.
#####...#
#####...#
.#.##.##.
...#..###
.####.#.#
...#....#
.#.#.#..#",

r"##.#.#.
###..##
##.#.##
.###...
.###...
##.#.##
###..##
##.#.#.
...###.
.#.###.
##.#.#.
###..##
##.#.##",

r".####..##
.#.##..##
##.######
.....##..
###..##..
....#..#.
##.######
....####.
.#.#....#",

r"#.#..#.#.##..##
........#.#####
...##....#..##.
.#.##.####.###.
########.#.#.##
##.##.##..##..#
..#..#..##..#.#
..#..#..##..#.#
##.##.##..##..#",

r"..#...#.#######
#.##....#..##..
..###..########
#....#.##..##..
#....####..##..
..###..########
#.##....#..##..
..#...#.#######
..####.###....#
#####.###.####.
.##..####......
...##...##.##.#
#...###..######",

r"#..#...#..#
#......#..#
.##......#.
####..#.###
..##..##...
#.##.#.#..#
#...##.###.
##.#...##..
##..###...#
##..###...#
##.#...##..
#...##.###.
#.##.#.#..#",

r".########....##..
###.##.###.##..##
..#.##.#..##.##.#
.##....##..######
##.#.##.##..####.
##.#..#.##.#....#
..######.........",

r".##..##.#.#..#.
#......########
#########......
#########.####.
###..###.#.##.#
##.######.####.
##.##.##.......
.######..#....#
..####...##..##
########..#..#.
..#..#..##.##.#
#.#..#.###....#
...##....#....#
...##....#.##.#
#......##..##..
#########.#..#.
...........##..",

r"#....#..#.#.##.
...##.#...#####
...##.#....####
#....#..#.#.##.
..###.###...##.
#..##..##.##..#
##...#......##.
.####.#..#.....
.##.##.#....##.
.#..#..#.##.##.
##.##.###......
#..#.....##....
#.#..#..#..####
.###.#.#...####
...#....#######
..#.#.#...#.##.
...#........##.",

r".......##.......#
#..###....###..#.
####.######.#####
....###..###.....
.##..#....#..##..
.##..#....#..##..
#####.####.######
.##.###..###.##.#
.##.#..##..#.##..
#..#........#..##
####.##..##.#####
###.##.##.##.###.
....###..###.....",

r".....#........#
.....#........#
..###...#.#.#..
##.#.##....#..#
###..#.#.####.#
.###....#..#..#
...#.#.#.#..##.
...#.#.#.#..##.
.###....#.....#",

r"#####.#
.....##
#####..
#####..
#..####
####.##
....#..
....#..
####.##
#..####
######.",

r"##.##.#######
.###.#...##..
###.##...##..
..#.#....##..
#..###.######
.#.#.#.#....#
...#...##..##
..##.##......
..##.####..##
.##.##.##..##
.#####.######",

r"....##...
..#....#.
....##...
####..###
####..###
##.#..#.#
..#....#.
###.....#
..#....#.
..##..##.
...####..",

r"#########....
##.##.####...
.#....#..#...
#.#..#.##.#..
.##..##..#...
.########....
#.####.#.....
#..##..#.##..
########.....
.######.#####
.######...#..
..####..##...
#.####.#.####
#.####.#.....
...##......##",

r"...##.##.....#.
..###.##.....#.
....##.#..###.#
..####.#.###.#.
##...##..###.#.
##.####...##..#
###.##.####.#.#
...#....#.#....
##.#####..##.##
....##.#.###...
##.#.####.#.###
......##..##.##
####.#..####..#",

r".##..#.##
#..##.##.
#..##.###
.##..#.##
####..##.
....#.###
.##...#..",

r"...#..##..#..#..#
.#.#..##..#..#..#
#......#.#.##.#.#
#...####.#.##.#.#
...###.###....###
...#.#...######..
#..####..........
..##..##.#.##.#.#
##.#.#..#..##..#.",

r"####.#..#.####.
.##..####..##..
##.##.##.##.###
..###.#..###...
##.#..##..#.##.
##..######..##.
...###..###...#
...###..###...#
##..######..##.
##.#..##..#.##.
..###.#..###...",

r"..##..##..#.##..#
.####.###########
.#..#.##........#
.####.##..#..#..#
##..##.#.#.##.#.#
#....#..###..###.
......#..........",

r"#.#..##.###..##
####.#.#..#..#.
..##.#..###..##
.#..##..##.##.#
.#..#...#######
##.#.#####.##.#
#.#.......#..#.
#.#...#........
#...#...#######
#.#...#.#.#..#.
.#...#..#.#..#.
##.#..##..#..#.
####..##..#..#.",

r"####..#...###..#.
.##.#.#.##.######
..#.##...######.#
##.##..##.#####.#
#.##....#........
#.##....#........
##.##...#.#####.#
..#.##...###...##
..#.##...###...##
##.##...#.#####.#
#.##....#........
#.##....#........
##.##..##.#####.#",

r"####..###.######.
#....##.####..###
#....##.####..###
####..###.######.
#..#.####.#.##.#.
#####....#..##..#
.#.#...#...#.##..
...##.#.#..#..#..
#.#.#..#.#......#
#.####.##........
#.###...#..#..#..
.#####.#.########
....#..#...####..
.#..#.#.##.####.#
..##..#.#.#....#.
.##.#..###.####.#
.#....#..########",

r"#..##...#..#....#
####.#.#....#..#.
.#.....###.######
..####.##.##.##.#
..####.##.##.##.#
.#.....##########
####.#.#....#..#.
#..##...#..#....#
#...######.######
##..#.#...##....#
#....###.####..##
#...##..###.####.
.###..#....#.##.#",

r"..##.#..#.##...
..#.#....#.#...
..#.#....#.#...
..##.#..#.##...
.####.##.####.#
#.#.######.#.##
#.#.#.##.#.#.##
..###.##.###.##
#..##.##.##..##
##.##.##.##.##.
##..##..##..##.",

r"#.###.##.#.#.##.#
#..####....######
##...##..###....#
####..#.###.#..#.
##.##...#.#..#...
...#...##.#..##..
###.#.#.#.#..##..
..#..##.##.#.##.#
.##.####.#.......
.##.####.#.......
..#..##.##.#.##.#
###.#.#.#.#..##..
...#...##.#..##..",

r"....##...
....##...
####.#..#
.##.#.#..
..#.###.#
.##.#.###
#####.###
.##.###..
#####.##.",

r".#.#......#######
###.###....#.####
#...#.#.##...####
.###.#..##...####
#####....####....
#.##..##.#...####
#.#.###.#.....##.
.##.###.#..#.....
###..#..##.#.####
..#.#.#..#.#.....
.###....#........
####.#..##..#....
#.##....#..#.####
.###.###.#..#####
.##..#.#..#######",

r"...#.####
....###.#
#.#.#...#
#.#.#...#
....#.#.#
...#.####
##.#..#.#
##..#....
.#....###
##...#.##
##...#.##
.#....###
##..#....
##.#..#.#
...#.####
....#.#.#
#.#.#...#",

r"##..##..#######
..######.......
....##....####.
....##....####.
#..#..#..######
#.#....#.##..##
###....########
###....###....#
.#......#..##..
...####...#..#.
#.#....#.######
#.#####..##..##
..#....#..#..#.
####..#########
...#..#...#..#.",

r"####...####
####...####
##...#....#
.####..####
...##..#...
.##.#.#..#.
##..#..#..#
#.#..##.#.#
.#..##..#.#
##.#####...
####.####..
####.####..
##.#####...
.#..##..#.#
..#..##.#.#",

r"#.#.#.#.###
#...#.#.###
.######.#.#
####......#
..#.##.###.
.##...###..
.##...###..
..#.##.###.
####......#
.######.#.#
#...#.#.###",

r"##..#.#
...#.##
...#.##
.#..#.#
####..#
.#####.
.#####.",

r"#.##.#.##....
#######..####
....#.#.##..#
..##..##..##.
......###....
.#..#.###....
#######.##..#
########.....
.#..#.##.####
#.##.#...####
#....#.#.#..#
##..##...####
##..##.######",

r"#..##.#..##
##.##.#.###
####....#..
####....#..
##.##.#.###
#..##.#..##
#...#....#.
...##.####.
##..##...##
.#...#.#...
.###.#.##.#
#..###..##.
#..###...#.
.###.#.##.#
.#...#.#...",

r"..#.####.#..##.
.....##.....###
..#.#..#.#...#.
##.#....#.####.
.#........#.#.#
...##..##...##.
###.####.###..#
###......######
####.##.####.##",

r"...##....####....
########..##..###
########......###
#......#..##..#..
#.#..#.#..##..#.#
#.#..#.########.#
...##...######...
.#....#.######.#.
#..##..##.##.###.
#########....####
##....###....###.",

r"..#.##..######..#
..#.##..######..#
##.##..#..##..#..
#...#.#..####..#.
.....#..######..#
.#.##.####..####.
#...#.#.#....###.
##...#####..#####
..#..#.#..##..#.#",

r".......####
##..##.#.#.
##..###.#..
#.##.##..##
#....##..##
.####.##...
#######....
##..###....
.#..#.##...
.#..#.#..##
......###..
..##....#..
##..####.##",

r".##....#.
####....#
.##..##..
...##..##
...##..##
.##..##..
####....#",

r"#.#.#.#.###
...##.....#
.######.##.
......#.##.
.#####.#...
.#...##...#
.#...##...#
.#####.#...
.#....#.##.
.######.##.
...##.....#
#.#.#.#.###
##.#.##.#.#
.#.##..#.#.
.#....###..
.#....###..
.#.##..#.#.",

r"#....#.####..
#.####.###..#
.#..#.#..#.#.
#.#..#....##.
#######..####
.#.#####.###.
.#.#####.###.
#######..####
#.#..#....##.
.#..#.#..#.#.
#.########..#
#....#.####..
#....#.####..",

r".#...###.##.#
.......######
.##.##..#..##
....#.#...#..
.##.###..#.##
.##.####..#.#
####.###.#..#
#..#.#....###
....##.....#.
....##.....#.
#..#.#....###",

r".##.##..#......
.#.#.....##.###
.#.#.....##.###
.##.##..#......
...#..##..#.#.#
#####.#....#.##
#####.###.#.###
..#.#.#.#..##..
..#.#.###..#...",

r"#.#...##.##..
####.##.#####
#...#....#.##
##..####.##..
##.#.#..#....
.##..##.#..##
.#..#.##.####
.#..#.....#..
#######.#####
##.##..#.....
##.##........
#######.#####
.#..#.....#..
.#..#.##.####
.##..##.#..##
##.#.#..#....
##..####.##..",

r"..#.###..###.#..#
..#.###..###.#..#
......#..#.......
.#..##....##..#..
.#.#...##...#.#.#
..#.#......#.#...
##............##.
#.##........##.##
..###.#..#.###...
.#....#..#....#..
#...#.#..#.#...##
######.##.######.
.#.##########.#.#
##.#.#.##.#.#.##.
.#.#.##..##.#.#..
#.############.##
#####......###.#.",

r"#..#...
..#####
.#....#
.#..###
#...###
.#.####
.#.#...
...####
.#.##..
..###..
...##..
...##..
..###..
.#.##..
...####",

r"#.#######....#.#.
#..#.##.##......#
...#.##.##......#
#.#######....#.#.
#.#######....#.#.
...#.##.##......#
#..#.##.##......#
#.#######....#.#.
##.#.#######....#
###.###...##.#...
#..#....#####....
..#.#..##.......#
###.######.#.#.#.
.#.#####.........
...##.#..######..
.###.#.##..#####.
#...#.#..#.###.##",

r"#...#.#
####...
####...
#...#.#
.....##
..##.##
.##...#
..##...
#.##.##
..###..
..###..
#.##.##
..##...
..#...#
..##.##",

r".#..#..##.#...##.
.#..#..##.#...##.
#.#.#####..####.#
....#.##....#.##.
.##.##.#.####....
###..###.....####
#.##.#..##.......",

r".#.#.###.#.##
#.###..#.#.##
#####.#.###..
#####.#.###..
#.###..#.#.##
.#.#.###.#.##
#....##..#...
#...####..###
##.#.###..#.#
##..#...#.###
##..#...#.###
##.#.###..#.#
#...####..###
#....##..#...
.#.#.###.####",

r".##.##.
#..#.#.
######.
#..#...
#..#..#
#..#..#
#..##..
######.
#..#.#.",

r".##.#.##.#..#.##.
.##.#.##.#..#.##.
###.#.##.#.##....
#.#.##.....#.####
##..#.##.#######.
....#.##.#.###..#
..#......#.##....
.#......###.#.##.
#..#.###.#...#..#",

r"...##...#..##
#.####.###...
...##.....#..
.#.##.#..##..
##....###....
........#..##
#......####..
#..##..#.....
###..###..###
.######..#...
#.####.....##",

r"##....##..##..##.
...#####.#..#.###
####.##.######.##
..##...#..##..#..
##.#...########..
..##....##..##...
..#.#############
..####....##....#
###....##....##.#
..##.###.#..#.###
##...#.#......#.#
##..##.###..###.#
...#.#...####...#
...#...##.##.##..
###..##.##..##.##
..#.#..##....##..
..###.###....###.",

r"........#.###
..##...#####.
.####.#.##..#
.####.#..#.#.
##..####..#.#
##..##..#...#
##..##..#...#
##..####..#.#
.####.#..#.#.
.####.#.##..#
..##...#####.
........#.###
######.#.#..#
##..####..###
#....#..#....
#.##.#.#....#
##.####.###.#",

r"#.#..#.#.#.#.####
###..#####....##.
#..##..##..###..#
#.######.###.#..#
########..#..####
.#.##.#.##...####
#.####.#.#..##..#
##....##..#..#..#
..####...##..#..#
#.####.#.#.......
...##...###..#..#
.#....#.###..#..#
..#..#..#.#..####",

r"...#...###.#.#...
.......#....#..##
.##.....##.####..
.#..######.#.##..
.#####.....###...
.#...###..###..##
###.#..#....##...
.#.####.#.#..#.##
#.####..##.#.####
#.#.##..##.#.####
.#.####.#.#..#.##",

r".##.#....#..#..#.
#....####....#...
#.#...##...#.####
.#...####...#.###
#.##.####.##.#.##
.#.##.##.##.#.#.#
#..###..###..##.#
#..###..###..##.#
.#.##.##.##.#.#.#
#.##.####.##.#.##
.#...####...#.###",

r".#..#.##...
.##..###...
####..##.##
.......#.##
.##.#.#....
....####...
.##.#...#..
####.#.##..
.##..##..##",

r".#.####..
#..######
##.##....
.##.##...
.##.##...
##.##....
#..######
.#.####..
#..#..###
.#...####
##.......
##....#..
#..#...##
.###..###
#...#...#
.#..#....
#.###....",

r".##.###.###.###
.#####.##.#..##
.#####.##.#..##
.##.###.#######
.#.##....##.###
##.###..#...###
.#...#.#####.##
##.###...######
.#.#.##....##..
##.#.#.##..#...
###.##..##.#...
#####....###...
#....#......###",

r".###...#.###.
#...#.#.....#
#...#.#.....#
.###...#.###.
.#..#....#..#
#..###.##.#..
#.##..#..#...
#.######...#.
#.######...#.
#.##..#..#...
##.###.##.#..
.#..#....#..#
.###...#.###.",

r"......####.#...#.
......##.##..#.#.
.#..#..#.####...#
.#..#..##.##.####
#.##.###..##..###
#######.##..###..
#.##.#....##..#..
..##...###.######
......##..##...##
......#.#...#.###
......#.#...#.###
......##..##.#.##
..##...###.######",

r"#..#..#..
.....####
.##.#...#
.##.#...#
.....####
#..#..#..
##.##.#..
#..#..#..
......#.#
.##...#..
####...#.
#####.#.#
#..#..#..
.##..#...
#####....
#..#.####
.......#.",

r".....#.#.
###...#..
..#..#.##
..###.##.
##.##....
##.##....
..###.##.
..#..#.##
###...#..
....##.#.
...######",

r"...##.##.#..####.
...##.##.#..####.
#.####..#..#.....
#..........#....#
..#.##...#..#..#.
.#...##..#.#.##.#
.#.#..##...##..##
#...#...#########
.#.##.#..###....#",

r"..#.##.
....###
.....##
##...##
...#...
##..##.
###....
...##.#
##..#..
..##..#
..##..#
##..#.#
...##.#",

r"#..#..#......#..#
#..#..#......#..#
###.######.#..#.#
.#.####.#.#####..
##..####..###.#..
.##...###.#.#.##.
...#....#.#.#####
##.#..####..#....
##.#..###..#..##.
##.#..###..#..##.
##.#..####..#....
...#....#.#.#####
.##...###.#.#.##.
##...###..###.#..
.#.####.#.#####..
###.######.#..#.#
#..#..#......#..#",

r"..#...#.#.##.
.###.#....##.
.##.#..######
#.##.##.#....
#######..####
#.#.##..#....
##.#..#.##..#
..#....######
......#######
......#######
..#....######
#..#..#.##..#
#.#.##..#....
#######..####
#.##.##.#....
.##.#..######
.###.#....##.",

r"##..#..#..#
...#...##..
###..##..##
..#..##..#.
###.####.##
###..##..##
##.#.##.#.#
##.######.#
..########.",

r"##..###
####.##
.#.#...
.....##
..#.###
..#.###
###....
##.....
##...##
..#....
##.#...
###.#..
..#.###",

r"..##....#
...####.#
...##..##
#####..##
...######
####....#
.....##..
...#.##.#
##.#....#",

r"####...#.#.......
#.#.####.####...#
..#.##.#...##....
..#.##.#...##....
#.#.####.####...#
####...#.#.......
#..#.#..#.#.#.##.
.#....#....#..###
..#.##.#...#.#.#.
..#.##.#...#.#.#.
.##...#....#..###
#..#.#..#.#.#.##.
####...#.#.......",

r"#.######.
#.#....##
.#..#.###
.#..#.###
#.#....##
#.######.
......#.#
.#....#.#
#.######.
#.#....##
.#..#.###",

r".##...#..#...##..
##..#.####.#..###
#......#.......##
##..#..##..#..###
..###..##..###...
#..#..####..#..##
#.##.#....#.##.##
#.##.#....#.##.##
.###.#....#.###..",
    ];

    let s = r"
.##...#..#...##..
##..#.####.#..###
#......#.......##
##..#..##..#..###
..###..##..###...
#..#..####..#..##
#.##.#....#.##.##
#.##.#....#.##.##
.###.#....#.###..".trim();

    let ans: usize = fields.iter().map(|field| solve(field)).flat_map(|(row, col)| [row * 100, col]).sum();

    println!("ans = {}", ans);
}

fn solve(field: &str) -> (usize, usize) {
    let row = find_mirror_center_row(field);
    let col = find_mirror_center_row(&transpose(field).to_string());

    (row, col)
}

fn find_mirror_center_row(field: &str) -> usize {
    /*
    [1,2,3,4,5]
    fold #0 - across 1/2 row: [1?2, discard]
    fold #1 - across 2/3 row: [1?4, 2?3, discard]
    fold #2 - across 3/4 row: [2?5, 3?4, discard]
    fold #3 - across 4/5 row: [4?5, discard]

    [1,2,3,4,5,6,7,8]
    fold #0 - across 1/2 row: [1?2, discard]
    fold #1 - across 2/3 row: [1?4, 2?3, discard]
    fold #2 - across 3/4 row: [2?5, 3?4, 1?6, discard]
    fold #3 - across 4/5 row: [4?5, 3?6, 2?7, 1?8, discard]
    fold #4 - across 5/6 row: [5?6, 4?7, 3?8, discard]
    fold #5 - across 6/7 row: [6?7, 5?8, discard]
    fold #6 - across 7/8 row: [7?8, discard]
     */
    let lines = field.lines().collect::<Vec<_>>();
    let rows = lines.len();

    for fold in (0..((rows - 1) / 2)).rev().chain(((rows - 1) / 2)..(rows - 1)).rev() {
        if fold < rows / 2 {
            let fold_size = fold + 1;

            let fold1 = lines.iter().take(fold_size);
            let fold2 = lines.iter().skip(fold_size).take(fold_size).rev();

            if fold1.zip(fold2).all(|(a, b)| a == b) {
                return fold + 1
            }
        } else {
            let fold_size = rows - (fold + 1);

            // the mutation does not make any sense whatsoever, but FU, it's rust!
            let fold1 = lines.iter().skip(rows - (fold_size * 2)).take(fold_size);
            let fold2 = lines.iter().skip(rows - fold_size).take(fold_size).rev();

            if fold1.zip(fold2).all(|(a, b)| a == b) {
                return fold + 1
            }
        }
    }

    0
}

fn transpose(field: &str) -> String {
    let cols = field.lines().next().unwrap().len();

    (0..cols).map(|col| field.lines().map(|line| line.chars().nth(col).unwrap()).collect::<String>()).collect::<Vec<_>>().join("\n")
}
