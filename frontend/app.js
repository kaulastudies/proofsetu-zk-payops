const runDemoBtn = document.getElementById("runDemoBtn");

const proofStatus = document.getElementById("proofStatus");
const stellarStatus = document.getElementById("stellarStatus");
const workflowStatus = document.getElementById("workflowStatus");

const steps = [
  document.getElementById("step1"),
  document.getElementById("step2"),
  document.getElementById("step3"),
  document.getElementById("step4"),
];

function resetSteps() {
  steps.forEach((step, index) => {
    if (index === 0) {
      step.classList.add("active");
    } else {
      step.classList.remove("active");
    }
  });

  proofStatus.textContent = "Not generated";
  stellarStatus.textContent = "Pending";
  workflowStatus.textContent = "Pending";
}

function activateStep(index) {
  steps[index].classList.add("active");
}

runDemoBtn.addEventListener("click", () => {
  resetSteps();

  setTimeout(() => {
    activateStep(1);
    proofStatus.textContent = "Generated";
    workflowStatus.textContent = "Proof submitted";
  }, 700);

  setTimeout(() => {
    activateStep(2);
    stellarStatus.textContent = "Verified";
    workflowStatus.textContent = "Verified";
  }, 1400);

  setTimeout(() => {
    activateStep(3);
    workflowStatus.textContent = "Eligible for settlement";
  }, 2100);
});
