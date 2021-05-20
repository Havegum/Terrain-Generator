<aside class="controls">
  <h2>Settings</h2>
  <hr>
  <section class="settings">
    <slot/>
  </section>
</aside>


<style>
.controls {
  padding: 8px;
  color: #f2f2f2;
  background-color: #212121;
  height: 100%;
  overflow-y: hidden;
  flex-direction: column;
  display: flex;
}

.settings {
  overflow-y: auto;
  display: grid;
  align-items: start;
  justify-content: start;
  grid-gap: 8px;

  grid-template-columns: repeat(auto-fit, minmax(500px, 1fr));
}

h2 {
  font-size: 18px;
  font-weight: normal;
}

hr {
  border: none;
  border-top: 1px solid #434647;
  margin: 2px 0 1px;
}

section :global(section) {
  margin-top: 8px;
  display: grid;
  grid-template-columns: minmax(10ch, 1fr) 4fr minmax(12ch, 1fr);
  grid-gap: 14px 8px;
  border: 1px solid #434647;
  border-radius: 8px;
  padding: 2px 8px 16px;
}

section :global(section > *) {
  grid-column: 2 / 3;
}

section :global(section > .col-1) {
  grid-column: 1 / 2;
  text-align: right;
}

section :global(section > .col-3) {
  grid-column: 3 / 4;
}

section :global(section > hr) {
  grid-column: 1 / 4;
  border: none;
  border-top: 1px solid #434647;
  margin-left: -8px;
  margin-right: -8px;
}

section :global(section > hr + h4) {
  margin-top: -.75em;
  margin-bottom: .25em;
}


section :global(h3),
section :global(h4) {
  font-weight: normal;
  font-size: inherit;
  font-size: .75em;
  opacity: 0.75;
  text-transform: uppercase;
  margin-top: 4px;
  margin-bottom: -4px;
  grid-column: 1 / -1;
}

section :global(label) {
  display: block;
}

section :global(input) {
  width: 100%;
}

.settings :global(button) {
  font-size: inherit;
  border: none;
  padding: 7px;
  border-radius: 4px;
  background-color: hsla(0, 0%, 100%, 0.1);
  color: inherit;
  cursor: pointer;
}

.settings :global(button:hover) {
  background-color: hsla(0, 0%, 100%, 0.2);
}

</style>
