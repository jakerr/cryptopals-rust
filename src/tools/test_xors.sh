output=$(cargo run -q ICE < test_input.txt)
expect="0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"

if [[ $expect == $output ]]; then
  echo "OK"
  exit 0
else
  echo "BAD"
  exit 1
fi
