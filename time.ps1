$stopwatch = [system.diagnostics.stopwatch]::startnew()
.\target\debug\dedupe.exe -d "C:\python35"
$stopwatch.stop()
$exe_time = $stopwatch.elapsed.totalseconds
write-host "`n" $exe_time
write-host "Enter message for log: "
$msg = read-host
write-output "$exe_time `t: $msg" >> time.log
