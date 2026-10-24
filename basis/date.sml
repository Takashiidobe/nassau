structure Date =
struct
  datatype weekday = Mon | Tue | Wed | Thu | Fri | Sat | Sun
  datatype month = Jan | Feb | Mar | Apr | May | Jun | Jul | Aug | Sep | Oct | Nov | Dec

  datatype date =
    DATE of {year : int, month : month, day : int, hour : int, minute : int, second : int,
             wday : weekday, yday : int, isDst : bool option, offset : Time.time option}

  exception Date

  val weekdays = [Mon, Tue, Wed, Thu, Fri, Sat, Sun]
  val months = [Jan, Feb, Mar, Apr, May, Jun, Jul, Aug, Sep, Oct, Nov, Dec]

  fun weekdayIndex d = #1 (valOf (List.find (fn (_, x) => x = d) (ListPair.zip (List.tabulate (7, fn i => i), weekdays))))
  fun monthIndex m = #1 (valOf (List.find (fn (_, x) => x = m) (ListPair.zip (List.tabulate (12, fn i => i), months))))
  fun weekdayOf i = List.nth (weekdays, i)
  fun monthOf i = List.nth (months, i)

  val dayNames = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"]
  val monthNames = ["January", "February", "March", "April", "May", "June", "July",
                    "August", "September", "October", "November", "December"]

  fun daysFromCivil (y, m, d) =
    let
      val y = if m <= 2 then y - 1 else y
      val era = y div 400
      val yoe = y - era * 400
      val mp = (m + 9) mod 12
      val doy = (153 * mp + 2) div 5 + d - 1
      val doe = yoe * 365 + yoe div 4 - yoe div 100 + doy
    in
      era * 146097 + doe - 719468
    end

  fun civilFromDays days =
    let
      val z = days + 719468
      val era = z div 146097
      val doe = z mod 146097
      val yoe = (doe - doe div 1460 + doe div 36524 - doe div 146096) div 365
      val doy = doe - (365 * yoe + yoe div 4 - yoe div 100)
      val mp = (5 * doy + 2) div 153
      val d = doy - (153 * mp + 2) div 5 + 1
      val m = if mp < 10 then mp + 3 else mp - 9
      val y = yoe + era * 400
    in
      (if m <= 2 then y + 1 else y, m, d)
    end

  val bias = 4503599627370496.0

  fun floorReal n =
    let
      val rounded = if n < 0.0 then (n - bias) + bias else (n + bias) - bias
    in
      if rounded > n then rounded - 1.0 else rounded
    end

  fun floorSeconds t = floorReal (Time.toReal t)

  fun fromSeconds (secs, isDst, offset) =
    let
      val dayCount = floorReal (secs / 86400.0)
      val days = Prim.realFloor dayCount
      val rest = Prim.realFloor (secs - dayCount * 86400.0)
      val (y, m, d) = civilFromDays days
    in
      DATE {year = y, month = monthOf (m - 1), day = d, hour = rest div 3600,
            minute = rest mod 3600 div 60, second = rest mod 60,
            wday = weekdayOf ((days + 3) mod 7), yday = days - daysFromCivil (y, 1, 1),
            isDst = isDst, offset = offset}
    end

  fun fieldSeconds (DATE {year, month, day, hour, minute, second, ...}) =
    Prim.intToReal (daysFromCivil (year, monthIndex month + 1, day)) * 86400.0
    + Prim.intToReal (hour * 3600 + minute * 60 + second)

  fun zoneAt secs =
    let val packed = Prim.timeZone secs in (packed div 2, packed mod 2 = 1) end

  fun localOffset () =
    let val (east, _) = zoneAt (floorSeconds (Time.now ())) in Time.fromSeconds (~east) end

  fun fromTimeUniv t = fromSeconds (floorSeconds t, SOME false, SOME Time.zeroTime)

  fun fromTimeLocal t =
    let
      val secs = floorSeconds t
      val (east, dst) = zoneAt secs
    in
      fromSeconds (secs + Prim.intToReal east, SOME dst, NONE)
    end

  fun toTime (d as DATE {offset, ...}) =
    let val wall = fieldSeconds d
    in
      case offset of
        SOME west => Time.fromReal (wall + floorSeconds west)
      | NONE =>
          let
            val (first, _) = zoneAt (wall - Prim.intToReal (#1 (zoneAt wall)))
          in
            Time.fromReal (wall - Prim.intToReal first)
          end
    end

  fun date {year, month, day, hour, minute, second, offset} =
    let
      val raw = DATE {year = year, month = month, day = day, hour = hour, minute = minute,
                      second = second, wday = Mon, yday = 0, isDst = NONE, offset = offset}
    in
      fromSeconds (fieldSeconds raw, NONE, offset)
    end

  fun year (DATE d) = #year d
  fun month (DATE d) = #month d
  fun day (DATE d) = #day d
  fun hour (DATE d) = #hour d
  fun minute (DATE d) = #minute d
  fun second (DATE d) = #second d
  fun weekDay (DATE d) = #wday d
  fun yearDay (DATE d) = #yday d
  fun isDst (DATE d) = #isDst d
  fun offset (DATE d) = #offset d

  fun compare (a, b) = Time.compare (toTime a, toTime b)

  fun pad2 n = if n < 10 then "0" ^ Int.toString n else Int.toString n
  fun pad3 n = if n < 10 then "00" ^ Int.toString n else if n < 100 then "0" ^ Int.toString n else Int.toString n
  fun abbrev name = String.substring (name, 0, 3)

  fun fmt format (d as DATE {year, month, day, hour, minute, second, wday, yday, ...}) =
    let
      val hour12 = if hour mod 12 = 0 then 12 else hour mod 12
      val wname = List.nth (dayNames, weekdayIndex wday)
      val mname = List.nth (monthNames, monthIndex month)
      fun directive c =
        case c of
          #"a" => abbrev wname
        | #"A" => wname
        | #"b" => abbrev mname
        | #"B" => mname
        | #"c" => fmt "%a %b %d %H:%M:%S %Y" d
        | #"d" => pad2 day
        | #"H" => pad2 hour
        | #"I" => pad2 hour12
        | #"j" => pad3 (yday + 1)
        | #"m" => pad2 (monthIndex month + 1)
        | #"M" => pad2 minute
        | #"p" => if hour < 12 then "AM" else "PM"
        | #"S" => pad2 second
        | #"U" => pad2 ((yday + 7 - (weekdayIndex wday + 1) mod 7) div 7)
        | #"w" => Int.toString ((weekdayIndex wday + 1) mod 7)
        | #"W" => pad2 ((yday + 7 - weekdayIndex wday) div 7)
        | #"x" => fmt "%m/%d/%y" d
        | #"X" => fmt "%H:%M:%S" d
        | #"y" => pad2 (year mod 100)
        | #"Y" => Int.toString year
        | #"Z" => ""
        | #"%" => "%"
        | _ => raise Date
      fun go (i, acc) =
        if i >= size format then String.concat (List.rev acc)
        else if String.sub (format, i) = #"%" then
          if i + 1 >= size format then raise Date
          else go (i + 2, directive (String.sub (format, i + 1)) :: acc)
        else go (i + 1, str (String.sub (format, i)) :: acc)
    in
      go (0, [])
    end

  fun toString d = fmt "%a %b %d %H:%M:%S %Y" d

  fun scan getc source =
    let
      fun skipWS state =
        case getc state of
          SOME (c, rest) => if Char.isSpace c then skipWS rest else state
        | NONE => state
      fun take (state, 0, acc) = SOME (String.implode (List.rev acc), state)
        | take (state, n, acc) =
            case getc state of
              SOME (c, rest) => take (rest, n - 1, c :: acc)
            | NONE => NONE
      fun lookup (name, names) =
        let
          fun find (i, []) = NONE
            | find (i, n :: rest) = if abbrev n = name then SOME i else find (i + 1, rest)
        in find (0, names) end
      fun num s = if List.all Char.isDigit (String.explode s) then Int.fromString s else NONE
    in
      case take (skipWS source, 24, []) of
        NONE => NONE
      | SOME (s, rest) =>
          let
            val field = fn (a, n) => String.substring (s, a, n)
            val wd = lookup (field (0, 3), dayNames)
            val mo = lookup (field (4, 3), monthNames)
            val dd = num (field (8, 2))
            val hh = num (field (11, 2))
            val mi = num (field (14, 2))
            val ss = num (field (17, 2))
            val yy = num (field (20, 4))
          in
            if String.sub (s, 3) = #" " andalso String.sub (s, 7) = #" " andalso String.sub (s, 10) = #" "
               andalso String.sub (s, 13) = #":" andalso String.sub (s, 16) = #":" andalso String.sub (s, 19) = #" "
            then
              case (wd, mo, dd, hh, mi, ss, yy) of
                (SOME _, SOME m, SOME d, SOME h, SOME n, SOME c, SOME y) =>
                  SOME (date {year = y, month = monthOf m, day = d, hour = h, minute = n,
                              second = c, offset = NONE}, rest)
              | _ => NONE
            else NONE
          end
    end

  fun fromString s = StringCvt.scanString scan s
end
