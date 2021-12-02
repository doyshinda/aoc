from collections import defaultdict


def get_input(filename):
    with open('/home/abe/misc/adventofcode/2019/{}'.format(filename), 'r') as _file:
        for line in _file:
            yield line.rstrip('\n')


#############
# Question 1
#############
def _calc_fuel(weight):
    return (weight // 3) - 2


def _calc_fuel_with_fuel(mass):
    total_fuel = _calc_fuel(mass)
    init_fuel = total_fuel
    remain = _calc_fuel(init_fuel)
    while remain > 0:
        total_fuel += remain
        remain = _calc_fuel(remain)

    return total_fuel


def one_part_a():
    fuels = []
    for line in get_input('1_a.input'):
        mass = int(line)
        if mass < 0:
            print(mass)
            continue
        fuels.append(_calc_fuel(mass))

    print(len(fuels))
    print(sum(fuels))


def one_part_b():
    fuels = []
    for line in get_input('1_a.input'):
        mass = int(line)
        if mass < 0:
            print(mass)
            continue
        fuels.append(_calc_fuel_with_fuel(mass))

    print(len(fuels))
    print(sum(fuels))


#############
# Question 2
#############
HALT = 99


def _get_int_codes():
    vals = None
    for line in get_input('2_a.input'):
        vals = [int(v) for v in line.split(',')]
        break

    return vals


def two_part_a(int_codes, for_realz=False, noun=12, verb=2):
    skip_val = 4
    idx = 0
    if for_realz:
        int_codes[1] = noun
        int_codes[2] = verb

    for opcode in int_codes:
        if idx % skip_val != 0:
            idx += 1
            continue

        if opcode == HALT:
            break

        r_pos_1 = int_codes[idx + 1]
        r_pos_2 = int_codes[idx + 2]
        s_pos_1 = int_codes[idx + 3]
        if opcode == 1:
            int_codes[s_pos_1] = int_codes[r_pos_2] + int_codes[r_pos_1]
        elif opcode == 2:
            int_codes[s_pos_1] = int_codes[r_pos_2] * int_codes[r_pos_1]
        else:
            raise Exception('oops')

        idx += 1

    return int_codes


def test_two():
    expected = [
        ([1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]),
        ([1, 0, 0, 0, 99], [2, 0, 0, 0, 99]),
        ([2, 3, 0, 3, 99], [2, 3, 0, 6, 99]),
        ([2, 4, 4, 5, 99, 0], [2, 4, 4, 5, 99, 9801]),
        ([1, 1, 1, 4, 99, 5, 6, 0, 99], [30, 1, 1, 4, 2, 5, 6, 0, 99])
    ]

    for e in expected:
        orig = [x for x in e[0]]
        res = two_part_a(e[0])
        assert res == e[1], '%s %s %s' % (orig, res, e[1])


# print(two_part_a(_get_int_codes(), for_realz=True))
# test_two()

def two_part_b():
    for noun in range(0, 100):
        for verb in range(0, 100):
            result = two_part_a(_get_int_codes(), for_realz=True, noun=noun, verb=verb)
            if result[0] == 19690720:
                print('noun = %d, verb = %d, answer: %d' % (noun, verb, 100 * noun + verb))
                print('result: %s' % result)
                print()


#############
# Question 3
#############
def _get_wires():
    wires = []
    for line in get_input('3_a.input'):
        yield line


def _wire_coords(wire):
    for val in wire.split(','):
        direction = val[0]
        amount = int(val[1:])
        yield direction, amount


CACHE = {}


def _start_end_range(c1, c2):
    key = (c1[0], c1[1], c2[0], c2[1])
    if key in CACHE:
        # print('cache_hit')
        return CACHE[key]

    x1 = c1[0]
    x2 = c2[0]
    if x1 > x2:
        temp = x2
        x2 = x1
        x1 = temp

    y1 = c1[1]
    y2 = c2[1]
    if y1 > y2:
        temp = y2
        y2 = y1
        y1 = temp

    x_range = set(list(range(x1, x2 + 1)))
    y_range = set(list(range(y1, y2 + 1)))

    CACHE[key] = (x_range, y_range)

    return x_range, y_range


def _cal_manhattan_distance(src, dst):
    return abs(src[0] - dst[0]) + (abs(src[1] - dst[1]))


def three_part_a():
    wires = _get_wires()
    wires = [
        # 'R8,U5,L5,D3',
        # 'U7,R6,D4,L4',
        'R75,D30,R83,U83,L12,D49,R71,U7,L72',
        'U62,R66,U55,R34,D71,R55,D58,R83',
        # 'R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51',
        # 'U98,R91,D20,R16,D67,R40,U7,R15,U6,R7'
    ]

    dir_map = {
        'R': 1,
        'L': -1,
        'U': 1,
        'D': -1
    }

    coords = []
    # for wire in _get_wires():
    for wire in wires:
        curr_x = 0
        curr_y = 0
        wire_coords = []
        for direction, amount in _wire_coords(wire):
            multiplier = dir_map[direction]
            delta = multiplier * amount
            if direction in ['U', 'D']:
                curr_y += delta
            else:
                curr_x += delta

            wire_coords.append((curr_x, curr_y))
        coords.append(wire_coords)

    crossings = []

    source_wire = coords[0]
    trace_wire = coords[1]

    print(len(source_wire))

    idx = 0
    prev_source = (0, 0)
    for coord in source_wire:
        print(idx)
        # if idx == 20:
        #     return
        source_x_range, source_y_range = _start_end_range(prev_source, coord)
        # print('%r -> %r: %s %s' % (prev_source, coord, source_x_range, source_y_range))

        prev_trace = (0, 0)
        for trace_coord in trace_wire:
            trace_x_range, trace_y_range = _start_end_range(prev_trace, trace_coord)
            # print('\t%r -> %r: %s %s' % (prev_trace, trace_coord, trace_x_range, trace_y_range))

            for x_val in trace_x_range:
                for y_val in trace_y_range:
                    if x_val in source_x_range and y_val in source_y_range:
                        crossings.append((x_val, y_val))

            prev_trace = trace_coord

        prev_source = coord
        idx += 1

    min_dist = 10000000000000000000
    closest = (0, 0)
    for c in crossings:
        if c == (0, 0):
            continue
        man_dist = _cal_manhattan_distance((0, 0), c)
        if man_dist < min_dist:
            min_dist = man_dist
            closest = c

    print(min_dist, closest)


def _get_pos(val, iterable, prev, curr):
    if curr > prev:
        min_val = min(iterable)
        diff = val - min_val
        return diff

    max_val = max(iterable)
    diff = max_val - val
    return diff


def three_part_b():
    wires = _get_wires()
    wires = [
        # 'R8,U5,L5,D3',
        # 'U7,R6,D4,L4',
        # 'R75,D30,R83,U83,L12,D49,R71,U7,L72',
        # 'U62,R66,U55,R34,D71,R55,D58,R83',
        # 'R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51',
        # 'U98,R91,D20,R16,D67,R40,U7,R15,U6,R7'
    ]

    dir_map = {
        'R': 1,
        'L': -1,
        'U': 1,
        'D': -1
    }

    coords = []
    for wire in _get_wires():
    # for wire in wires:
        curr_x = 0
        curr_y = 0
        wire_coords = []
        for direction, amount in _wire_coords(wire):
            multiplier = dir_map[direction]
            delta = multiplier * amount
            if direction in ['U', 'D']:
                curr_y += delta
            else:
                curr_x += delta

            wire_coords.append((curr_x, curr_y))
        coords.append(wire_coords)

    crossings = []

    source_wire = coords[0]
    trace_wire = coords[1]

    print(len(source_wire))

    idx = 0
    prev_source = (0, 0)
    source_steps = 0
    source_seen = {(0, 0): 0}
    for coord in source_wire:
        print(idx)
        source_x_range, source_y_range = _start_end_range(prev_source, coord)
        # print('%r -> %r: %s %s [%s]' % (prev_source, coord, source_x_range, source_y_range, source_steps))

        # if idx == 20:
        #     return

        prev_trace = (0, 0)
        trace_steps = 0
        trace_seen = {prev_trace: 0}
        for trace_coord in trace_wire:
            trace_x_range, trace_y_range = _start_end_range(prev_trace, trace_coord)
            # print('\t%r -> %r: %s %s [%s]' % (prev_trace, trace_coord, trace_x_range, trace_y_range, trace_steps))

            for x_val in trace_x_range:
                for y_val in trace_y_range:
                    if x_val in source_x_range and y_val in source_y_range:
                        xidx = _get_pos(x_val, trace_x_range, prev_trace[0], trace_coord[0])
                        yidx = _get_pos(y_val, trace_y_range, prev_trace[1], trace_coord[1])
                        s_xidx = _get_pos(x_val, source_x_range, prev_source[0], coord[0])
                        s_yidx = _get_pos(y_val, source_y_range, prev_source[1], coord[1])
                        # print('\t\t%s %s %s %s' % (x_val, y_val, (source_steps + s_xidx + s_yidx), (trace_steps + xidx + yidx)))
                        crossings.append((x_val, y_val, source_steps + trace_steps + xidx + yidx + s_xidx + s_yidx))

            if trace_coord not in trace_seen:
                trace_steps += _cal_manhattan_distance(prev_trace, trace_coord)
                trace_seen[trace_coord] = trace_steps
            else:
                trace_steps = trace_seen[trace_coord]

            prev_trace = trace_coord

        if coord not in source_seen:
            source_steps += _cal_manhattan_distance(prev_source, coord)
            source_seen[coord] = source_steps
        else:
            source_steps = source_seen[coord]
        prev_source = coord
        idx += 1

    min_dist = 10000000000000000000
    closest = (0, 0)
    for c in crossings:
        print(c)
        if c[0] == 0 and c[1] == 0:
            continue
        dist = c[2]
        if dist < min_dist:
            min_dist = dist
            closest = (c[0], c[1])

    print(min_dist, closest)


# three_part_b()


#############
# Question 4
#############
def _has_double(str_num):
    if not str_num:
        return False
    prev = str_num[0]
    for curr in str_num[1:]:
        if prev == curr:
            return True

        prev = curr

    return False


def _is_increasing(str_num):
    prev = int(str_num[0])
    for curr in str_num[1:]:
        if int(curr) < prev:
            return False

        prev = int(curr)

    return True


def _is_valid(str_num):
    return _has_double(str_num) & _is_increasing(str_num)


def four_part_a():
    valid_range = range(231832, 767346)
    count = 0
    for num in valid_range:
        if _is_valid(str(num)):
            count += 1

    print(count)


def _not_adjacent(str_num):
    invalid = set()
    for idx, curr in enumerate(str_num[1:-1], 1):
        if curr in invalid:
            continue

        prev = str_num[idx - 1]
        nxt = str_num[idx + 1]
        # print('%s %s %s' % (prev, curr, nxt))
        if prev == curr == nxt:
            invalid.add(curr)

    new_num = str_num
    for i in invalid:
        new_num = new_num.replace(i, '')

    return _has_double(new_num)


def _is_valid_b(str_num):
    if _is_valid(str_num) and _not_adjacent(str_num):
        return True

    return False


def four_part_b():
    valid_range = range(231832, 767346)
    count = 0
    for num in valid_range:
        if _is_valid_b(str(num)):
            count += 1

    print(count)


# four_part_a()
# print(_is_increasing('111110'))
# print(_is_valid_b('112233'))
# print(_is_valid_b('123444'))
# print(_is_valid_b('111122'))
# print(_is_valid_b('111123'))
four_part_b()