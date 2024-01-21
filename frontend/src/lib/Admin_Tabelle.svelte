<script>
  import AdminPlanRow from "./Admin_Plan_Row.svelte";



  let row_count = 2;


  let rows = [["7b", "5-6", "D", "Mustermann", "Horst", "D", "400", "Vertretung", "-"],["8b", "5-6", "D", "Mustermann", "Horst", "D", "400", "Vertretung", "-"]]



  function add_row() {

    row_count++;

    rows.push(["0b", "5-6", "D", "Mustermann", "Horst", "D", "400", "Vertretung", "-"])


  }

  function delete_row(row) {

    rows.splice(row,1)

    console.log(rows)

    row_count--;
  }


  $: print_rows(rows);

  function print_rows(input) {

    console.log(input)
  }

</script>

<div class="table-section">
  <div class="table-container">
    
    <div class="button-container">
      {#each Array(row_count) as _, i}
        <button class="delete-button" on:click={() => {delete_row(i)}}>X</button>
      {/each}
    </div>

    <table class="table">
      <tr>
        <th>Klasse</th>
        <th>Stunde</th>
        <th>altes Fach</th>
        <th>abwesend</th>
        <th>Vertreter</th>
        <th>neues Fach</th>
        <th>Raum</th>
        <th>Typ</th>
        <th>Info</th>
      </tr>

      {#each Array(row_count) as _, i}
        <AdminPlanRow id={i} bind:row_list={rows[i]}  /> 
      {/each}

    </table>
    
    <div class="button-container" style="opacity: 0;">
      {#each Array(row_count) as _, i}
        <button class="delete-button">X</button>
      {/each}
    </div>

  </div>

  <button class="add-button" on:click={() => {add_row()}}>Zeile hinzufügen</button>
</div>

<style>

  
  .table {
    border-collapse:separate;

    border-spacing: 0px;

    border-radius: 10px;

    border: solid;

    border-width: 1px;

    width: 90%;


    overflow: hidden;

    


    box-shadow: 0px 4px 4px 2px rgba(0, 0, 0, 0.2);
    
  } 
  .table-container {

    width: 100%;


    margin-top: 15vh;

    margin-bottom: 4vh;

    
    display: flex;


    justify-content: space-between;

  }

  .add-button {

    margin: 0 auto; 
  }


  .button-container {

    display: flex;

    flex-direction: column;


    margin-top: 45px;

  } 

  .delete-button {

    height: 35px;
  }
</style>
