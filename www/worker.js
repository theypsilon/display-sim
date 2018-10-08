self.onmessage = (event) => {
    var url = event.data.url;
    var xhr = new XMLHttpRequest();
    xhr.open('GET', url, false);
    xhr.responseType = 'arraybuffer';
    xhr.send(null);
    self.postMessage({buffer: new Uint8Array(xhr.response)});
}