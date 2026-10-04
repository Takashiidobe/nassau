# <span id="section:0"></span>The `Timer` structure

---

#### Synopsis

<span id="TIMER:SIG:SPEC"></span>
<span id="Timer:STR:SPEC"></span>

```sml
signature TIMER
structure Timer :> TIMER
```

The `Timer` structure provides facilities for measuring the passing of wall clock (real) time and the amount of time the running process has had the CPU (user time), has been active in the OS kernel (system time), and has spent on garbage collection (GC time).

---

#### Interface

<span id="SIG:TIMER.cpu_timer:TY:SPEC"></span>
<span id="SIG:TIMER.real_timer:TY:SPEC"></span>
<span id="SIG:TIMER.startCPUTimer:VAL:SPEC"></span>
<span id="SIG:TIMER.checkCPUTimes:VAL:SPEC"></span>
<span id="SIG:TIMER.checkCPUTimer:VAL:SPEC"></span>
<span id="SIG:TIMER.checkGCTime:VAL:SPEC"></span>
<span id="SIG:TIMER.totalCPUTimer:VAL:SPEC"></span>
<span id="SIG:TIMER.startRealTimer:VAL:SPEC"></span>
<span id="SIG:TIMER.checkRealTimer:VAL:SPEC"></span>
<span id="SIG:TIMER.totalRealTimer:VAL:SPEC"></span>

```sml
type cpu_timer
type real_timer
val startCPUTimer : unit -> cpu_timer
val checkCPUTimes : cpu_timer -> {
nongc : {
usr : Time.time,
sys : Time.time
},
gc : {
usr : Time.time,
sys : Time.time
}
}
val checkCPUTimer : cpu_timer -> {usr : Time.time, sys : Time.time}
val checkGCTime : cpu_timer -> Time.time
val totalCPUTimer : unit -> cpu_timer
val startRealTimer : unit -> real_timer
val checkRealTimer : real_timer -> Time.time
val totalRealTimer : unit -> real_timer
```

#### Description

<span id="SIG:TIMER.cpu_timer:TY"></span>**`type`**` cpu_timer`
**`type`**` real_timer`  
Type `real_timer` is the type of wall clock (real) timers, and `cpu_timer` is the type of CPU timers.

<span id="SIG:TIMER.startCPUTimer:VAL"></span>**`val`**` startCPUTimer `**`:`**` unit `**`->`**` cpu_timer`  
This returns a CPU timer that measures the time the process is computing (has control of the CPU) starting at this call.

<span id="SIG:TIMER.checkCPUTimes:VAL"></span>
`checkCPUTimes ``timer`` `  
returns the CPU time used by the program since the `timer` was started. The time is split into time spent in the program (`nongc`) and time spent in the garbage collector (`gc`). For each of these categories, the time is further split into time spent by code in _user space_ (`usr`) and time spent in the operating system on behalf of the program (`sys`). The total CPU time used by the program will be the sum of these four values.

<span id="SIG:TIMER.checkCPUTimer:VAL"></span>
`checkCPUTimer ``timer`` `  
returns the user time (`usr`) and system time (`sys`) that have accumulated since the timer `timer` was started. This function is equivalent to

        fun checkCPUTimer ct = let
                  val {nongc, gc} = checkCPUTimes ct
                  in {
                    usr = Time.+(#usr nongc, #usr gc),
                    sys = Time.+(#sys nongc, #sys gc)
          } end


<span id="SIG:TIMER.checkGCTime:VAL"></span>
`checkGCTime ``timer`` `  
returns the user time spent in garbage collection since the timer `timer` was started. This function is equivalent to

        fun checkGCTime ct = #usr(#gc(checkCPUTimes ct))


<span id="SIG:TIMER.totalCPUTimer:VAL"></span>**`val`**` totalCPUTimer `**`:`**` unit `**`->`**` cpu_timer`  
This returns a CPU timer that measures the time the process is computing (has control of the CPU) starting at some system-dependent initialization time.

<span id="SIG:TIMER.startRealTimer:VAL"></span>**`val`**` startRealTimer `**`:`**` unit `**`->`**` real_timer`  
This returns a wall clock (real) timer that measures how much time passes, starting from the time of this call.

<span id="SIG:TIMER.checkRealTimer:VAL"></span>
`checkRealTimer ``rt`` `  
returns the amount of (real) time that has passed since the timer `rt` was started.

<span id="SIG:TIMER.totalRealTimer:VAL"></span>**`val`**` totalRealTimer `**`:`**` unit `**`->`**` real_timer`  
This returns a wall clock (real) timer that measures how much time passes, starting from some system-dependent initialization time.

#### Examples

```repl
Timer.startRealTimer ();;
```

#### See Also

> [`Time`](time.md#Time:STR:SPEC)

#### Discussion

The accuracy of the user, system, and GC times depends on the resolution of the system timer and the function call overhead in the OS interface. In particular, very small intervals might not be reported accurately.

On a Unix system, the user and system time reported by a CPU timer do not include the time spent in child processes.

> **Implementation note:**
>
> Some operating systems may lack the ability to measure CPU time consumption, in which case the real time should be returned instead.
