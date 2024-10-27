<script lang="ts">
  import { render_element, type Element } from "$lib/wasm/engine";

  // State
  let elements = $state<Element[]>([]);

  function add_element(): void {
    elements.push({
      tag: "div",
      text: null,
      children: [
        { tag: "h1", text: "Hello World!", children: null },
        { tag: "p", text: "What an amazing rendering experience.", children: null },
      ],
    });
  }

  function remove_element(index: number): void {
    elements = elements.filter((_, i) => i !== index);
  }
</script>

<button onclick={add_element}>Add element</button>

{#each elements as element, index (index)}
  <div>
    {@html render_element(element)}
    <button onclick={() => remove_element(index)}>Remove element</button>
  </div>
{/each}
