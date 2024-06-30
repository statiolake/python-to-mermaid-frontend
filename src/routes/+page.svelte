<script lang="ts">
  import mermaid from 'mermaid';
  import Mermaid from '$lib/Mermaid.svelte';
  import init, { convert } from '$lib/wasm/pkg/python_to_mermaid_frontend.js';
  import { onMount } from 'svelte';

  onMount(() => {
    init();
    mermaid.initialize({
      startOnLoad: false,
      flowchart: {
        useMaxWidth: false
      }
    });
  });

  let input = '';
  let diagrams = [];
  let output = '';
  let error = '';

  const onClick = () => {
    error = '';
    try {
      output = '';
      diagrams = convert(input);
      for (const diagram of diagrams) {
        output += `# ${diagram.name}\n`;
        output += '\n';
        output += '```mermaid\n';
        output += diagram.diagram;
        output += '```\n';
        output += '\n';
      }
    } catch (e) {
      error = e as string;
    }
  };
</script>

<head>
  <link rel="preconnect" href="https://fonts.gstatic.com/" crossorigin="" />
  <link
    rel="stylesheet"
    as="style"
    href="https://fonts.googleapis.com/css2?display=swap&amp;family=Noto+Sans%3Awght%40400%3B500%3B700%3B900&amp;family=Space+Grotesk%3Awght%40400%3B500%3B700"
  />

  <title>Python to Mermaid Converter</title>
  <link rel="icon" type="image/x-icon" href="data:image/x-icon;base64," />

  <script src="https://cdn.tailwindcss.com?plugins=forms,container-queries"></script>
</head>
<body>
  <div
    class="relative flex size-full min-h-screen flex-col bg-[#111a22] dark group/design-root overflow-x-hidden"
    style="font-family: 'Space Grotesk', 'Noto Sans', sans-serif;"
  >
    <div class="layout-container flex h-full grow flex-col">
      <div class="px-40 flex flex-1 justify-center py-5">
        <div
          class="layout-content-container flex-col w-[512px] max-w-[512px] py-5 max-w-[960px] flex-1"
        >
          <div class="flex flex-wrap justify-between gap-3 p-4">
            <p class="text-white tracking-light text-[32px] font-bold leading-tight min-w-72">
              Convert Python to Mermaid Diagrams
            </p>
          </div>
          <div class="flex flex-1 flex-wrap items-end gap-4 px-4 py-3">
            <label class="flex flex-col h-300 min-w-40 flex-1">
              <p class="text-white text-base font-medium leading-normal pb-2">Python Code</p>
              <textarea
                placeholder="Python code here..."
                class="form-input flex h-full w-full min-w-0 flex-1 resize-none rounded-xl text-white focus:outline-0 focus:ring-0 border border-[#344d65] bg-[#1a2632] focus:border-[#344d65] min-h-36 placeholder:text-[#93adc8] p-[15px] text-base font-normal leading-normal"
                bind:value={input}
              />
            </label>
          </div>

          <div class="flex px-4 py-3">
            <button
              class="flex min-w-0 cursor-pointer items-center justify-center rounded-xl h-10 px-4 flex-1 bg-[#1980e6] text-white text-sm font-bold leading-normal tracking-[0.015em]"
              on:click={onClick}
            >
              <span class="truncate">Convert</span>
            </button>
          </div>

          {#if error}
            <div class="flex px-4 py-3">
              <p class="text-red-500 text-base font-medium leading-normal pb-2">{error}</p>
            </div>
          {/if}

          <div class="flex flex-1 flex-wrap items-end gap-4 px-4 py-3">
            <label class="flex flex-col h-300 min-w-40 flex-1">
              <p class="text-white text-base font-medium leading-normal pb-2">
                Markdown with Mermaid Diagrams
              </p>
              <textarea
                placeholder="Markdown with Mermaid diagrams will appear here..."
                class="form-input flex h-full w-full min-w-0 flex-1 resize-none rounded-xl text-white focus:outline-0 focus:ring-0 border border-[#344d65] bg-[#1a2632] focus:border-[#344d65] min-h-36 placeholder:text-[#93adc8] p-[15px] text-base font-normal leading-normal"
                bind:value={output}
                readonly
              />
            </label>
          </div>

          <div class="flex px-4 py-3">
            <button
              class="flex min-w-[84px] cursor-pointer items-center justify-center rounded-xl h-10 px-4 flex-1 bg-[#243647] text-white text-sm font-bold leading-normal tracking-[0.015em]"
              on:click={() => navigator.clipboard.writeText(output)}
            >
              <span class="truncate">Copy</span>
            </button>
          </div>

          {#if diagrams}
            {#each diagrams as diagram}
              <div class="flex flex-col px-4 py-3">
                <p class="text-white text-base font-medium leading-normal pb-2">{diagram.name}</p>
                <div class="flex w-full items-center justify-center">
                  <Mermaid name={diagram.name} diagram={diagram.diagram} />
                </div>
              </div>
            {/each}
          {/if}
        </div>
      </div>
    </div>
  </div>
</body>
