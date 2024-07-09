<script>

  import { get } from "svelte/store";

  import { admin_rows, changed_rows } from "../stores";

  export let id;


  let row_list;

  admin_rows.subscribe((value) => {

    row_list = value[id];
  })


  let changed = [];

  changed_rows.subscribe((value) => {
    changed = value;
  })


  function row_to_changed_list() {

    for (let i = 0; i < changed.length; i++) {

      if (changed[i].id == row_list.id) {

        return; 
      }
    }

    console.log("changed")
    
    changed_rows.set([...changed, row_list]);
  }

  async function update_changed_row() {

    console.log("row changed with on:changed")

    console.log(get(changed_rows))

    for (let i = 0; i < changed.length; i++) {

      if (changed[i].id == row_list.id) {

        return; 
      }
    }

    changed_rows.set([...changed, row_list]);


    // send whole changed list!
    const res = await fetch('/update', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(get(changed_rows))
    })

    console.log("updated")

    changed_rows.set([]);

  }


</script>

<tr class="row">

  <td><input type="text" bind:value={row_list.class} on:change={update_changed_row} class="input-field" style="width: 90%;"></td>

  <td style='white-space: nowrap'>
    <input type="number" bind:value={row_list.start_hour} on:change={update_changed_row} class="input-field" style="width: 3vw; display: inline-block;">

    <input type="number" bind:value={row_list.end_hour} on:change={update_changed_row} class="input-field" style="width: 3vw; display: inline-block;">
  </td>

  <td><input type="text" bind:value={row_list.old_fach} on:change={update_changed_row} class="input-field" style="width: 90%;"></td>

  <td><input type="text" bind:value={row_list.away} on:change={update_changed_row} class="input-field" style="width: 90%;"></td>

  <td><input type="text" bind:value={row_list.sub} on:change={update_changed_row} class="input-field" style="width: 90%;"></td>

  <td><input type="text" bind:value={row_list.new_fach} on:change={update_changed_row} class="input-field" style="width: 90%;"></td>

  <td><input type="text" bind:value={row_list.room} on:change={update_changed_row} class="input-field" style="width: 90%;"></td>

  <td><input type="text" bind:value={row_list.typ} on:change={update_changed_row} class="input-field" style="width: 90%;"></td>

  <td><input type="text" bind:value={row_list.info} on:change={update_changed_row} class="input-field" style="width: 90%;"></td>

</tr>

<style>
  .input-field {

    width: auto;
  }

  ::placeholder {

    color: black;

    opacity: 1;

  }

  

</style>
