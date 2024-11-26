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
            type: 'doc',
            label: 'Getting Started',
            id: 'getting-started',
        },
        {
            type: 'category',
            label: 'Setup',
            items: [
                'setup/rust-setup',
                'setup/ios-setup',
                'setup/android-setup',
                'setup/react-native-setup',
                'setup/flutter-setup'
            ]
        },
        {
            type: 'category',
            label: 'Supported Adapters',
            items: [
                'adapters/overview',
                'adapters/circom',
                'adapters/halo2',
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