<script>
  
  export let is_edit = false;

  let table = [
    [{"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"}, 
     {"fach":"Deutsch", "teacher":"Hr. Mustermann", "room": "442"}, 
     {"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"},
     {"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"},
     {"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"}],
    [{"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"}, 
     {"fach":"Deutsch", "teacher":"Hr. Mustermann", "room": "442"}, 
     {"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"},
     {"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"},
     {"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"}],
    [{"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"}, 
     {"fach":"Deutsch", "teacher":"Hr. Mustermann", "room": "442"}, 
     {"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"},
     {"fach":"Frei"},
     {"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"}],
    [{"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"}, 
     {"fach":"Deutsch", "teacher":"Hr. Mustermann", "room": "442"}, 
     {"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"},
     {"fach":"Frei"},
     {"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"}],
    [{"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"}, 
     {"fach":"Deutsch", "teacher":"Hr. Mustermann", "room": "442"}, 
     {"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"},
     {"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"},
     {"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"}],

  ];

</script>


<div>
  <tr class="row">
    <th class="table-head"></th>
    <th class="table-head">Montag</th>
    <th class="table-head">Dienstag</th>
    <th class="table-head">Mittwoch</th>
    <th class="table-head">Donnerstag</th>
    <th class="table-head">Freitag</th>
  </tr>


  {#if is_edit}
    

    {#each {length: 10} as _, i }
      <tr class="row">
        <td class="table-cell">{i + 1}. Stunde</td>

        {#if i < table.length}

          {#each table[i] as item}

            {#if item.fach == "Frei"}
              <td class="table-cell">
                <button class="add-container">
                  <img class="add" src="./src/assets/add.svg" alt="Add">
                </button>
              </td>

            {:else}
              <td class="table-cell"> 
                <b style="font-weight: 600;">{item.fach}</b>
                Hr. Mustermann
                <br>
                440 
              </td>
            {/if}
          {/each}



        {:else}

          {#each {length: 5} as _, i }
          
          <td class="table-cell">
          
              <button class="add-container">
                <img class="add" src="./src/assets/add.svg" alt="Add">
              </button>

          </td>

          {/each}
        {/if}
      </tr>
    {/each}
    


  {:else}



    {#each table as row, i (i)}
      <tr class="row">
      <td class="table-cell">{i + 1}. Stunde</td>
      {#each row as j}
          <td class="table-cell">

            {#if j.fach != "Frei"}
              <b style="font-weight: 600;">{j.fach}</b>
              Hr. Mustermann
              <br>
              440  
            {/if}  

          </td>
      {/each}

      </tr>
    {/each}


  {/if}

</div>


<style>
  .row {

    background-color: var(--secondary);

  }

  .table-head {

    background-color: var(--secondary);

    vertical-align: middle; 

    width: 12vw;

    height: 6.5vh;
  }

  .table-cell {

    vertical-align: center;

    text-align: center;

    height: 6.5vh;

  }

  
  .add-container {

    background-color: var(--accent);

    border: none;

    border-radius: 25px;

    
    width: 30px;
    height: 30px;

    
    margin: auto;

    padding: 0px;
  }

  .add {

    width: 30px;
    height: 30px;

  }

</style>
