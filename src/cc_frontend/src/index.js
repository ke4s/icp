
import { cc_backend } from "../../declarations/cc_backend";

async function hashPDF() {
  const fileInput = document.getElementById('pdfFile');

  if (fileInput.files.length > 0) {
    const file = fileInput.files[0];

    const arrayBuffer = await readFile(file);

    const hash = await hashData(arrayBuffer);
    return "SHA-256_" + hash;
  } else {
    alert('Please select a PDF file.');
  }

}

function readFile(file) {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();

    reader.onload = function (event) {
      resolve(event.target.result);
    };

    reader.onerror = function (event) {
      reject(event.target.error);
    };

    reader.readAsArrayBuffer(file);
  });
}

async function hashData(data) {
  const buffer = await crypto.subtle.digest('SHA-256', data);
  const hashArray = Array.from(new Uint8Array(buffer));
  const hashHex = hashArray.map(byte => byte.toString(16).padStart(2, '0')).join('');
  return hashHex;
}

document.querySelector("form").addEventListener("submit", async (e) => {
  e.preventDefault();

  var name = document.getElementById('name').value;
  var book_name = document.getElementById('book_name').value;
  var subject = document.getElementById('subject').value;
  var description = document.getElementById('description').value;

  if (name.trim() == '' || book_name.trim() == '' || description.trim() == '' || subject.trim() == '') {
    alert('Please fill out all required fields before proceeding.');
    return;
  }

  const hash = await hashPDF();

  const progressBar = document.getElementById('progressBar');
  const progressLabel = document.getElementById('progressLabel');

  progressBar.style.display = 'block';
  progressLabel.innerText = 'Uploading...';

  try {
    await new Promise(resolve => setTimeout(resolve, 2000));

    for (let i = 1; i <= 100; i++) {
      progressBar.value = i;
      await new Promise(resolve => setTimeout(resolve, 20));
    }

    const response = await cc_backend.update(hash, name, book_name, subject, description);
    alert(response);
  } catch (error) {
    alert('Error!');
  } finally {
    progressBar.style.display = 'none';
    progressLabel.innerText = '';
    progressBar.value = 0;
  }
});