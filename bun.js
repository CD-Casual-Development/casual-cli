import { $ } from 'bun';

// await $`cargo run -- project add -t "Test" -d "This is a test project" -c 1`
// await $`cargo run -- project add-task -p 1 -t "Test Task" -d "This is a test task" --minutes-estimated 240 --minute-rate 100`
// await $`cargo run -- project add-task -p 1 -t "Test Task 2" -d "This is a test task 2" --minutes-estimated 120 --minute-rate 100`
await $`cargo run -- project make-quote -p 1 -r "Cool comment" -d 400`
