window.addEventListener('load', ()=>{
    document.querySelectorAll('.copy-button').forEach((btn)=>{
        btn.addEventListener('click', async () => {
            await navigator.clipboard.writeText(window.location.protocol + '//' + window.location.host +  btn.getAttribute('data'))
        })
    })
})