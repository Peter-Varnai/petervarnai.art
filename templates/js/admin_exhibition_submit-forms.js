
// exhibitions related forms
document.getElementById('exhibitionForm').addEventListener('submit', async (e) => {
    e.preventDefault();

    const typeValue = Number(
        e.target.querySelector('input[name="type"]:checked').value
    );

    const formData = {
        title: e.target.name.value,
        start_date: e.target.start_date.value,
        till: e.target.till.value,
        location: e.target.location.value,
        link: e.target.link.value,
        type: typeValue,
    };

    console.log(formData)

    try {
        const response = await fetch('/exhibition', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(formData),
        });

        if (response.ok) {
            console.log('Exhibition added successfully');
            window.location.reload();
        } else {
            console.error('Failed to add exhibition');
        }
    } catch (err) {
        console.log()
        console.error('Network error:', err);
    }
});



document.querySelectorAll(".delete-exhib-btn").forEach(btn => {
    btn.addEventListener("click", () => {
        console.log('del button clicked')
        let json_id = JSON.stringify({ 'id': Number(btn.dataset.id) })
        console.log('sending delete project request.')
        fetch('/exhibition', {
            method: 'DELETE',
            headers: { 'Content-Type': 'application/json' },
            body: json_id
        }).then(r => {
            if (r.ok) window.location.reload();
            else console.error("Error deleting exhibition");
        });

    });
});


