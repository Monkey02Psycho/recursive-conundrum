function httpGetAsync(theUrl, callback) {
    var xmlHttp = new XMLHttpRequest();
    xmlHttp.onreadystatechange = function () {
        if (xmlHttp.readyState == 4 && xmlHttp.status == 200)
        // xmlHttp.setRequestHeader('Access-Control-Allow-Origin', '*')
        callback(xmlHttp.responseText);
    }
    xmlHttp.open("GET", theUrl, true); // true for asynchronous 
    xmlHttp.send(null);
}



window.setInterval(function () {
    httpGetAsync("https://9v7mhvuib8.execute-api.us-west-2.amazonaws.com/Beta/status", function (text) {
        console.log(text)
        data = JSON.parse(text)
        document.getElementById("ip").textContent = "IP: " + data.ip
        document.getElementById("status").textContent = "Status: " + data.state.Name
    });
    console.log("made request")
}, 1000);

function start() {
    httpGetAsync("https://9v7mhvuib8.execute-api.us-west-2.amazonaws.com/Beta/start", function (text) {
        console.log(text)
        data = JSON.parse(text)
        document.getElementById("ip").textContent = "IP: " + data.ip
        document.getElementById("status").textContent = "Status: " + data.state.Name
    });
    console.log("made request")
}
