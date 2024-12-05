

$inputfile = '.\inputdata.txt'
# $inputfile = '.\sampledata.txt'

$content = Get-Content $inputfile

$pattern = [regex]"(?:mul\(\d+,\d+\))|(do(?:n't)?\(\))"

function mul( $lr ){    
    return $lr[0] * $lr[1]
}

$structions = $pattern.Matches($content) 
$part1_sum  = 0
$part2_sum  = 0
$enabled = $true 

foreach ($item in @($structions.value)) {    
    switch ($item){
        {$_ -notlike "do*"} {$part1_sum += Invoke-Expression "$item"}
        "do()" {  $enabled = $true; continue}
        "don't()" { $enabled = $false}
        {$enabled} { $part2_sum += Invoke-Expression "$item"  }
        default {}
    }
}
[PSCustomObject]@{
    Part1 = $part1_sum
    Part2 = $part2_sum 
}
