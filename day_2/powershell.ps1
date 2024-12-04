$inputfile = '.\inputdata.txt'
# $inputfile = '.\sampledata.txt'

$lines = gc $inputfile
$safeCount = 0 
foreach ($line in $lines){
    $l = $line -split " "
    $deltas = @()
    $ascending = $true
    $descending = $true

    for ($i = 0; $i -lt $l.count -1; $i++ ){
        $deltas += ,([math]::abs([int]$l[$i] - [int]$l[$i+1]))
        $ascending = $ascending -AND [int]$l[$i] -lt [int]$l[$i+1]
        $descending = $descending -AND [int]$l[$i] -gt [int]$l[$i+1]        
    }

    $delta = ($deltas | Measure-Object -Maximum -Minimum)
    $d = ($delta.Maximum -le 3 -AND $delta.Minimum -gt 0)
    $isSafe = $d -AND ( $ascending -XOR $descending )

    $safecount += $isSafe ? 1 : 0 
    Write-Output "$($lines.IndexOf($line))`t| $($line.padright(22))`t| rate: $d`t| safe: $isSafe`t| asc: $ascending`t| desc: $descending"
    
}
$safecount 