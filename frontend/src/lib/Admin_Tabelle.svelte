<script>
  import AdminPlanRow from "./Admin_Plan_Row.svelte";
  import { admin_rows, changed_rows } from "../stores";
  import { onMount } from "svelte";

  export let day;



  let rows = [];

  admin_rows.subscribe((value) => {

    rows = value;
  })

  let changed = [];

  changed_rows.subscribe((value) => {

    changed = value;
  })

  


  onMount(async () => {
    await get_rows();
  });




  let row_count;

  $: row_count = rows.length;

  async function get_rows() {

    const res = await fetch('/get_rows', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          id: 1,
        }),
    })
    
    let db_rows = await res.json()

    console.log(db_rows[0].id);

  
    admin_rows.set(db_rows);
  }


  async function post_row() {

    const res = await fetch('/add_row', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          id: 0,
          day: 1,
          class: "0b",
          start_hour: 5,
          end_hour: 6,
          old_fach: "D",
          new_fach: "D",
          away: "Mustermann",
          sub: "Horst",
          room: "400",
          typ: "Vertretung",
          info: "-"
        }),
    })
        
    
  }

  async function add_row() {

    admin_rows.update((n) => [...n, {id:0, day: 1, class:"0b", start_hour:5, end_hour:6, old_fach:"D", away:"Mustermann",sub:"Horst", new_fach:"D", room:"400", typ:"Vertretung", info:"-"}])

    await post_row()

    get_rows()
  }

  async function update_row() {

    console.log(changed)

    // send whole changed list!
    const res = await fetch('/update', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(changed)
    })
  }

  async function delete_row(row) {

    console.log("row id: " + rows[row].id)
    

    const res = await fetch('/remove', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({id: rows[row].id}),
    })
    rows.splice(row,1)

    console.log(rows)

    admin_rows.set(rows)

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
        <AdminPlanRow id={i}  /> 
      {/each}

    </table>
    
    <div class="button-container" style="opacity: 0;">
      {#each Array(row_count) as _, i}
        <button class="delete-button">X</button>
      {/each}
    </div>

  </div>
  
  <div class="add-container">
    <button class="add-button" on:click={() => {add_row()}}>Zeile hinzufügen</button>
    <button class="add-button" on:click={() => {update_row()}}>Update</button>
  </div>
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

  .add-container {

    display: flex;

    justify-content: center;

    margin-bottom: 4vh;
  }

  .add-button {

    margin: 0 auto; 
  }


  .button-container {

    display: flex;

    flex-direction: column;


    margin-top: 45px;

    margin-left: 2.5%;

  } 

  .delete-button {

    height: 25px;

    width: 25px;

    margin-top: 5px;

    margin-bottom: 5px;
  }
</style>
