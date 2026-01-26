// projects related forms
document.getElementById('addProjectForm').addEventListener('submit', function(e) {
    e.preventDefault();
    const form = e.target;
    const formData = new FormData(form);

    fetch('/project', {
        method: 'POST',
        body: formData,
    })
        .then(response => {
            if (response.ok) {
                console.log("succesfully added project")
                window.location.reload()
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
    }).then(response => {
        if (response.ok) {
            console.log("succesfully edited project ")
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
        fetch('/project', {
            method: 'DELETE',
            headers: { 'Content-Type': 'application/json' },
            body: json
        }).then(r => {
            if (r.ok) {
                console.log("succesfully deleted project")
                window.location.reload();
            } else console.log('error deleting project', json);
        });
    })
})

