function pullEntities() {
  fetch("http://127.0.0.1:8000/results.json")
    .then(response => {
      if (!response.ok) {
        throw new Error('network response not ok');
      }
      return response.json();
    })
    .then(data => {
      console.log(data);
      const results_html = parseResult(data);
      document.getElementById("api-response-body").innerHTML = results_html.join("");
    })
    .catch(error => {
      console.log(error);
    })
}

function parseResult(data) {
  const results_html = data.results.map((result) => {
    return `
      <div class="pull-result pull-entity-${parameterize(result.entity.kind)} ${parameterize(result.rarity_category)}">
        <p>${result.entity.name}</p>
      </div>
    `
  });

  return results_html;
}

function parameterize(str) {
  return str.replace(/([a-z](?=[A-Z]))/g, '$1-').toLowerCase();
}

window.addEventListener("load", (_event) => {
  console.log("page is fully loaded");
  document.getElementById("btn-entity").addEventListener('click', function() { console.log("clicked entity") })
  document.getElementById("btn-recall").addEventListener('click', pullEntities)
});
