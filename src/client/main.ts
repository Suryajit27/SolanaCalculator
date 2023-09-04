import {
  establishConnection,
  establishPayer,
  checkProgram,
  calculate,
  displayResult,
} from "./calculatorprogram";

async function main() {
  let operation;
  let num1;
  let num2;

  if (process.argv.length !== 5) {
    throw new Error("Invalid input.");
  }

  process.argv.forEach(function (val, index, array) {
    switch (index) {
      case 2: {
        operation = val;
      }
      case 3: {
        num1 = parseInt(val);
      }
      case 4: {
        num2 = parseInt(val);
      }
    }
  });

  if (!operation || !num1 || !num2) {
    throw new Error("Invalid input.");
  }

  // Establish connection to the cluster
  await establishConnection();

  // Determine who pays for the fees
  await establishPayer();

  // Check if the program has been deployed
  await checkProgram();

  // call calculator program
  await calculate(operation, num1, num2);

  // Find out the result
  await displayResult(operation, num1, num2);
}

main().then(
  () => process.exit(),
  (err) => {
    console.error(err);
    process.exit(-1);
  }
);
