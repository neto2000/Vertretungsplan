<script>

  import AddMask from "./Add_Mask.svelte";

  
  export let is_edit = false;

  let show_mask = false;

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


  function mask() {

    show_mask = true;
  }

</script>


<div>

  {#if show_mask}

    <AddMask bind:show_mask={show_mask}/>
  {/if}

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
                <button class="add-container" on:click={mask}>
                  <img class="add" src="./src/assets/add.svg" alt="Add">
                </button>
              </td>

            {:else}
              <td class="table-cell">

                {#if is_edit}

                  <button class="edit-button">
                    <b style="font-weight: 600;">{item.fach}</b>
                    Hr. Mustermann
                    <br>
                    440

                                  
                  </button>

                  <button class="mini-edit-container" on:click={mask}>
                    <div class="edit-circle">                      
                      <img class="edit-pic" src="./src/assets/edit.svg" alt="Edit">
                    </div>

                  </button>

                {:else}

                  <b style="font-weight: 600;">{item.fach}</b>
                  Hr. Mustermann
                  <br>
                  440
                {/if}


                 
              </td>
            {/if}
          {/each}



        {:else}

          {#each {length: 5} as _, i }
          
          <td class="table-cell">
          
              <button class="add-container" on:click={mask}>
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

    position: relative;

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

  .edit-button {

    width: 100%;


    height: 6.5vh;

    text-align: center;


    border: none;

    background-color: rgba(0,0,0,0);

    padding: 0px 0px 0px 0px;



    font-size: 16px;

    font-family: "Rubik", sans-serif;

  } 
  

  .mini-edit-container {
    
    border: transparent;

    background-color: transparent;


    width: 100%;

    height: 6.5vh;

    z-index: 2;

    position: absolute;

    top: 0; 

    display: flex;
 
    transition: background-color 0.5s;
  }
  .mini-edit-container:hover {

    background-color: rgba(226, 237, 246, 0.5);

  } 

  .mini-edit-container:hover .edit-circle{

    opacity: 100%;



  }

  .edit-circle {

    display: flex;

    width: 30px;

    height: 30px;


    z-index: 2;
    

    border: none;

    border-radius: 20px;

    background-color: var(--accent);

    margin: auto;

    padding: 0px;


    opacity: 0%;

    transition: opacity 0.5s;

  }

  .edit-pic {
    
    width: 22px;
    height: 22px;


    margin: auto;

  }

</style>
