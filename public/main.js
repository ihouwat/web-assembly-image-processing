// Importing WA file. Since importing a file is an async operation, need async keyword
async function init() {

  // Importing WA file here
  let rustApp = null;
  
  try {
    rustApp = await import('../pkg'); // pkg directory created by web assembly tools plugin
  } catch(e) {
    console.error(e);
    return;
  }

  console.log(rustApp);

  const input = document.getElementById('upload');
  const fileReader = new FileReader();

  fileReader.onloadend = () => {
    const base64 = fileReader.result.replace(
      /^data:image\/(png|jpeg|jpg);base64,/, 'f'
    );
    console.log(input.files[0]);
    console.log(base64);
  }

  input.addEventListener('change', () => {
    fileReader.readAsDataURL(input.files[0]); // readAsDataURL converts file to text
  });
}

init();