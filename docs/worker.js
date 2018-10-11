self.onmessage = (event) => {
    var url = event.data.url;
    var xhr = new XMLHttpRequest();
    xhr.open('GET', url, false);
    xhr.responseType = 'blob';
    xhr.send(null);
    self.postMessage({blob: xhr.response});
}