<script lang="ts">
  import mermaid from 'mermaid';

  export let name: string;
  export let diagram: string;

  let container: HTMLSpanElement;

  async function renderDiagram() {
    const sanitizedName = name.replace(/\./g, "_");
    const { svg } = await mermaid.render(`mermaid_${sanitizedName}`, diagram);
    container.innerHTML = svg;
  }

  $: diagram && renderDiagram();
</script>

<main>
  <span bind:this={container}> </span>
</main>
