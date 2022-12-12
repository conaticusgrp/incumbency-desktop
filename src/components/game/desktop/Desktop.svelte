<script lang="ts">
  import { onMount } from "svelte"
  import App from "../../../App.svelte"
  import DesktopShortcut from "./DesktopShortcut.svelte"

  
  onMount(() => {
    const desktopShortcuts = document.getElementsByTagName("desktopShortcut")
    for (let i = 1; i < desktopShortcuts.length; i++) {
      ;(desktopShortcuts[i] as HTMLElement).style.gridRow = `${i} / ${i + 1}`
    }
  })
</script>

<main>
  <!-- <desktopShortcut></desktopShortcut> -->
  <DesktopShortcut
    name="Email"
    icon={"https://images.unsplash.com/photo-1670766552268-b8e06b420008?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=735&q=80"}
    badgeCount={2}
  />
  <DesktopShortcut
    name="Government Spending"
    icon={"https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fi.pinimg.com%2Foriginals%2F7b%2F0c%2Fe0%2F7b0ce0e9e5938afc0b3fb4e0ea64e6c0.png&f=1&nofb=1&ipt=d2efa2bed562eb8efafbd699109100294472a51e60fb725ceda3adebcea3fdfb&ipo=images"}
  />
  <DesktopShortcut
    name="Stocks"
    icon="https://cdn.discordapp.com/attachments/821000000000000000/821000000000000000/unknown.png"
    badgeCount={1}
  />
  <DesktopShortcut
    name="Contacts"
    icon="https://cdn.discordapp.com/attachments/821000000000000000/821000000000000000/unknown.png"
  />
  <DesktopShortcut
    name="Add/Remove Apps"
    icon="https://cdn.discordapp.com/attachments/821000000000000000/821000000000000000/unknown.png"
  />
</main>

<style>
  main {
    padding: 2rem;
    display: grid;
    grid-template-columns: 1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr;
    grid-template-rows: 1fr 1fr 1fr 1fr 1fr 1fr 1fr;
    height: 100%;
    width: 100%;
    background: #171717;
  }
</style>
