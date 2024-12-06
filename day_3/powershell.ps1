

$inputfile = '.\inputdata.txt'
$content = Get-Content $inputfile
$pattern = [regex]"(?:mul\(\d+,\d+\))|(do(?:n't)?\(\))"
function mul( $lr ){    
    return $lr[0] * $lr[1]
}

$structions = $pattern.Matches($content) 
$results = @{ Part1 = 0; Part2 = 0 }
$enabled = $true 

foreach ($item in @($structions.value)) {    
    switch ($item){
        { $_ -notlike "do*" } { $results.part1 += Invoke-Expression "$item" }
        "do()"     { $enabled = $true; continue }
        "don't()"  { $enabled = $false }
        { $enabled } { $results.part2 += Invoke-Expression "$item" }
        default {}
    }
}
[PSCustomObject]$results
