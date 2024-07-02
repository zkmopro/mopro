import type { SidebarsConfig } from '@docusaurus/plugin-content-docs';

const sidebars: SidebarsConfig = {
  docsSidebar: [
    {
      type: 'doc',
      label: 'Introduction',
      id: 'intro',
    },
    {
      type: 'doc',
      label: 'Prerequisites',
      id: 'prerequisites',
    },
    {
      type: 'category',
      label: 'Getting Started',
      items: [
        'getting-started/rust-setup',
        'getting-started/ios-setup',
        'getting-started/android-setup'
      ]
    },
    {
      type: 'doc',
      label: 'Mopro FFI',
      id: 'mopro-ffi',
    },
    {
      type: 'doc',
      label: 'Performance and Benchmarks',
      id: 'performance',
    },
    {
      type: 'doc',
      label: 'Community and Talks',
      id: 'community',
    },
    {
      type: 'doc',
      label: 'FAQ',
      id: 'FAQ',
    },
  ],
};

export default sidebars;