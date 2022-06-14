set -x

resim reset

OP1=$(resim new-account)
export privkey1=$(echo "$OP1" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export account1=$(echo "$OP1" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")

OP2=$(resim new-account)
export privkey2=$(echo "$OP2" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export account2=$(echo "$OP2" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")

export package=$(resim publish . | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")

CP_OP=$(resim call-function $package Chess instantiate_with_fen "rnb1kb1r/8/8/8/3nq3/8/4N3/RN2K2R b KQkq - 0 1")
export component=$(echo "$CP_OP" | sed -nr "s/└─ Component: ([[:alnum:]_]+)/\1/p")
export player=$(echo "$CP_OP" | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '1!d')

resim transfer 1 $player $account2

# At this point account1 is the White Team, and Account 2 is the Black Team
# resim set-default-account $account1 $privkey1
# resim call-method $component move_piece "A2" "A4" 1,$player
resim set-default-account $account2 $privkey2
resim call-method $component move_piece "E4" "E2" 1,$player

# resim set-default-account $account1 $privkey1
# resim call-method $component move_piece "B2" "B4" 1,$player
# resim set-default-account $account2 $privkey2
# resim call-method $component move_piece "B7" "B5" 1,$player

# resim set-default-account $account1 $privkey1
# resim call-method $component move_piece "C1" "A3" 1,$player
# resim set-default-account $account2 $privkey2
# resim call-method $component move_piece "C7" "C5" 1,$player

# resim set-default-account $account1 $privkey1
# resim call-method $component move_piece "B4" "C5" 1,$player
# resim set-default-account $account2 $privkey2
# resim call-method $component move_piece "D7" "D5" 1,$player