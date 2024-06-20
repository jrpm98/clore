fetch("https://clore.ai/webapi/marketplace/reset_container", {
    "headers": {
        "accept": "application/json",
        "accept-language": "zh-CN,zh;q=0.9",
        "content-type": "application/json",
        "priority": "u=1, i",
        "sec-ch-ua": "\"Google Chrome\";v=\"125\", \"Chromium\";v=\"125\", \"Not.A/Brand\";v=\"24\"",
        "sec-ch-ua-mobile": "?0",
        "sec-ch-ua-platform": "\"macOS\"",
        "sec-fetch-dest": "empty",
        "sec-fetch-mode": "cors",
        "sec-fetch-site": "same-origin",
        "cookie": "cloud-selection=community; spot-show-usd=true; clore_token=MTcxODQ4MDIzNV9PanFNSUFhYkFNa2s5eGVhUTJxUXd4cXF4ckFUQVc=; marketplace_query=eyJyZWxldmFudF9taW5fZ3B1IjoxLCJyZWxldmFudF9tYXhfZ3B1IjoxNiwicmVsZXZhbnRfZ3B1X21lbSI6MiwicmVsZXZhbnRfZ3B1X21heF9tZW0iOjEyOCwiZm9yX2xvZ19ncHVfbWF4IjoxMjgwLCJmb3JfbG9nX2dwdV9taW4iOjIsInJlbGV2YW50X21pbl9tZW0iOjQsInJlbGV2YW50X21heF9tZW0iOjEwMjQsImZvcl9sb2dfbWVtb3J5X21heCI6MTAyNDAsImZvcl9sb2dfcmVsaWFiaWxpdHkiOjg5MTI5LCJmb3JfbG9nX21lbW9yeV9taW4iOjE4OTAsInJlbGV2YW50X21pbl9yZWxpYWJpbGl0eSI6OTkuOTcsInNob3dfcmVudGVkX3NlcnZlcnMiOmZhbHNlLCJzaG93X29ubHlfb2MiOmZhbHNlLCJtYXJrZXRfY3VycmVuY3kiOiJDTE9SRS1CbG9ja2NoYWluIiwibWFya2V0X2dwdV9maWx0ZXIiOiJhbGwiLCJtYXJrZXRfY3VkYSI6IkFueSIsIm1hcmtldF9zb3J0IjoibWluIiwibWFya2V0X3R5cGUiOiJtYWlubGluZSJ9",
        "Referer": "https://clore.ai/marketplace",
        "Referrer-Policy": "strict-origin-when-cross-origin"
    },
    "body": "{\"order_id\":274087,\"token\":\"MTcxNjY1NDA2N19XeElVS0dJR2dpMGF4cDJzbmtOeDNWOUVyczg3amQ=\"}",
    "method": "POST"
}).then((res)=>{
    return res.json()
}).then((text)=>{
    console.log(text)
}).catch((e)=>{
    console.log(e)
});