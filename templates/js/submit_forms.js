// exhibitions related forms
document.getElementById('exhibitionForm').addEventListener('submit', async (e) => {
    e.preventDefault();

    const formData = {
        title: e.target.name.value,
        start_date: e.target.start_date.value,
        till: e.target.till.value,
        location: e.target.location.value,
        link: e.target.link.value,
        big_row: e.target.big_row.checked,
    };

    try {
        const response = await fetch('/exhibition', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(formData)
        });

    } catch (error) {
        console.error('Error:', error);
    }
});

document.querySelectorAll(".delete-exhib-btn").forEach(btn => {
    btn.addEventListener("click", () => {
        console.log('del button clicked')
        let json_id = JSON.stringify({ 'id': Number(btn.dataset.id) })
        console.log('sending delete project request :', json_id)
        fetch('/exhibition', {
            method: 'DELETE',
            headers: { 'Content-Type': 'application/json' },
            body: json_id
        }).then(r => {
            if (r.redirected) window.location = r.url;
            else location.reload();
        });

    });
});




// projects related forms
document.getElementById('addProjectForm').addEventListener('submit', function(e) {
    e.preventDefault();
    const form = e.target;
    const formData = new FormData(form);

    console.log(formData)
    fetch('/project', {
        method: 'POST',
        body: formData,
    })
        .then(response => {
            if (response.ok) {
                console.log("succesful form submission", response)
            } else {
                console.log("error while submitting form", response)
            }
        })
        .catch(error => {
            console.error('Error:', error);
        });
});

document.getElementById('editProjectForm').addEventListener('submit', function(e) {
    e.preventDefault();
    const form = e.target;
    const formData = new FormData(form);

    fetch('/project', {
        method: 'PUT',
        body: formData,
    })
        .then(response => {
            if (response.ok) {
                console.log("succesful form submission", response)
            } else {
                console.log("error while submitting form", response)
            }
        })
        .catch(error => {
            console.error('error:', error);
        });
});

document.querySelectorAll('.delete-project-btn').forEach(btn => {
    btn.addEventListener('click', function() {
        let json = JSON.stringify({
            'id': Number(btn.dataset.id),
            'folder_path': btn.dataset.folder,
        })
        console.log('sending request to delete project: ', json)
        fetch('/project', {
            method: 'DELETE',
            headers: { 'Content-Type': 'application/json' },
            body: json
        }).then(r => {
            console.log('answer :', r)
            if (r.ok) window.location = r.url;
            else console.log('error deleting project', json);
        });
    })
})

// SELECT WHICH PROJECT TO EDIT
document.querySelectorAll('.editProjectBtn').forEach(btn => {
    btn.addEventListener('click', function() {
        console.log('edit project button pressed', btn.dataset.id)
        fetch(`/project?no=${btn.dataset.id}`)
            .then(r => r.json())
            .then(project => {
                const form = document.getElementById('editProjectForm');
                form.querySelector('input[name="title"]').value = project.title;
                form.querySelector('input[name="date"]').value = project.date.substring(0, 7); // YYYY-MM
                form.querySelector('input[name="video_link"]').value = project.video_link || '';
                form.querySelector('input[name="medium"]').value = project.medium || '';
                form.querySelector('input[name="duration"]').value = project.duration || '';
                form.querySelector('textarea[name="concept"]').value = project.concept;
                form.querySelector('input[name="dir"]').value = project.dir;
                form.querySelector('input[name="id"]').value = project.id;

                console.log('Form populated with:', project);
            })
            .catch(err => console.log('Error:', err));
    })
})


// function deleteProjecct(id, path) {
//     let json = JSON.stringify({
//         'id': Number(id),
//         'folder_path': path,
//     })
//     console.log('sending request to delete project: ', json)
//     fetch('/project', {
//         method: 'DELETE',
//         headers: { 'Content-Type': 'application/json' },
//         body: json
//     }).then(r => {
//         console.log('answer :', r)
//         if (r.redirected) window.location = r.url;
//         else console.log('error deleting project', json);
//     });
// }
