# Runs Cargo 100 times to test the program's output

output_file="output_results.txt"
> "$output_file" # Clear the output file if it exists

for i in {1..100}; do
  cargo run >> "$output_file"
done

echo "Done. Results are in $output_file."
