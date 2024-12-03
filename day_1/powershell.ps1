$inputfile = '.\input.txt'

$sum = 0 
$A = @()
$B = @()

get-content $inputfile | ForEach-Object { 
    $t_a, $t_b = $_ -split "   "; 
    $A += ,$t_a; 
    $B += ,$t_b
}

$A = $A | sort-object 
$B = $B | sort-object 


(0..($A.count - 1)) | ForEach-Object { $sum += [math]::abs( $A[$_] - $B[$_] )}

$sum