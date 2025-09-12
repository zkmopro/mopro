export interface ComponentData {
  id: string;
  name: string;
  displayName?: string[]; // For multi-line text display
  description: string;
  position: { x: number; y: number };
  width: number;
  height: number;
  color: string;
  textColor?: string;
  category: string;
  links?: string[];
}

export interface LayerData {
  id: string;
  name: string;
  position: { x: number; y: number };
  width: number;
  height: number;
  color: string;
  textColor: string;
}

export const architectureLayers: LayerData[] = [
  {
    id: 'roles',
    name: 'roles',
    position: { x: 20, y: 120 },
    width: 1040,
    height: 100,
    color: '#E1F5FE',
    textColor: '#0277BD'
  },
  {
    id: 'app',
    name: 'app',
    position: { x: 20, y: 250 },
    width: 140,
    height: 350,
    color: '#FFCDD2',
    textColor: '#D32F2F'
  },
  {
    id: 'mobile-native-lib',
    name: 'mobile native lib',
    position: { x: 180, y: 250 },
    width: 150,
    height: 350,
    color: '#FFCDD2',
    textColor: '#D32F2F'
  },
  {
    id: 'mobile-rust-lib',
    name: 'mobile rust lib',
    position: { x: 350, y: 250 },
    width: 180,
    height: 350,
    color: '#FFF3E0',
    textColor: '#F57C00'
  },
  {
    id: 'middleware',
    name: 'middleware',
    position: { x: 550, y: 250 },
    width: 180,
    height: 350,
    color: '#F3E5F5',
    textColor: '#9C27B0'
  },
  {
    id: 'proving-system',
    name: 'proving system',
    position: { x: 750, y: 250 },
    width: 140,
    height: 350,
    color: '#F3E5F5',
    textColor: '#9C27B0'
  },
  {
    id: 'circuits',
    name: 'circuits',
    position: { x: 910, y: 250 },
    width: 150,
    height: 350,
    color: '#E8F5E8',
    textColor: '#4CAF50'
  }
];

export const architectureComponents: ComponentData[] = [
  // Roles layer
  {
    id: 'user',
    name: 'User',
    description: 'End users with no specialized knowledge required ("no brain")',
    position: { x: 90, y: 172 },
    width: 110,
    height: 40,
    color: '#BBDEFB',
    textColor: '#1976D2',
    category: 'roles'
  },
  {
    id: 'app-dev',
    name: 'App dev',
    description: 'App developers who write Swift/Kotlin/React Native/Flutter code',
    position: { x: 350, y: 172 },
    width: 110,
    height: 40,
    color: '#BBDEFB',
    textColor: '#1976D2',
    category: 'roles'
  },
  {
    id: 'tooling-infra-dev',
    name: 'Tooling/ Infra dev',
    description: 'Tooling/ Infra developers who write Rust code',
    position: { x: 610, y: 172 },
    width: 110,
    height: 40,
    color: '#BBDEFB',
    textColor: '#1976D2',
    category: 'roles'
  },
  {
    id: 'circuits-dev',
    name: 'Circuits dev',
    description: 'Circuits developers who write Circom/Noir code',
    position: { x: 870, y: 172 },
    width: 110,
    height: 40,
    color: '#BBDEFB',
    textColor: '#1976D2',
    category: 'roles'
  },

  // App layer
  {
    id: 'ios-app',
    name: 'iOS app',
    description: 'Native iOS applications built with Swift/Objective-C',
    position: { x: 30, y: 320 },
    width: 120,
    height: 50,
    color: '#FFFFFF',
    textColor: '#000000',
    category: 'app',
    links: ['https://developer.apple.com/ios/']
  },
  {
    id: 'android-app',
    name: 'Android app',
    description: 'Native Android applications built with Kotlin/Java',
    position: { x: 30, y: 380 },
    width: 120,
    height: 50,
    color: '#FFFFFF',
    textColor: '#000000',
    category: 'app',
    links: ['https://developer.android.com/']
  },
  {
    id: 'react-native-app',
    name: 'react native app',
    description: 'Cross-platform mobile apps using React Native framework',
    position: { x: 30, y: 440 },
    width: 120,
    height: 50,
    color: '#FFFFFF',
    textColor: '#000000',
    category: 'app',
    links: ['https://reactnative.dev/']
  },
  {
    id: 'flutter-app',
    name: 'flutter app',
    description: 'Cross-platform mobile apps using Flutter framework',
    position: { x: 30, y: 500 },
    width: 120,
    height: 50,
    color: '#FFFFFF',
    textColor: '#000000',
    category: 'app',
    links: ['https://flutter.dev/']
  },

  // Mobile native lib layer
  {
    id: 'mopro-swift-lib',
    name: 'mopro swift lib',
    description: 'Swift bindings for iOS, generated from Rust via UniFFI',
    position: { x: 190, y: 320 },
    width: 130,
    height: 50,
    color: '#FFFFFF',
    textColor: '#000000',
    category: 'mobile-native-lib',
    links: ['https://github.com/zkmopro/mopro-swift-package']
  },
  {
    id: 'mopro-kotlin-lib',
    name: 'mopro kotlin lib',
    description: 'Kotlin bindings for Android, generated from Rust via UniFFI',
    position: { x: 190, y: 380 },
    width: 130,
    height: 50,
    color: '#FFFFFF',
    textColor: '#000000',
    category: 'mobile-native-lib',
    links: ['https://github.com/zkmopro/mopro-kotlin-package']
  },
  {
    id: 'mopro-react-native-lib',
    name: 'mopro react native lib',
    displayName: ['mopro react', 'native lib'],
    description: 'React Native package wrapping the native libraries',
    position: { x: 190, y: 440 },
    width: 130,
    height: 50,
    color: '#FFFFFF',
    textColor: '#000000',
    category: 'mobile-native-lib',
    links: ['https://github.com/zkmopro/mopro-react-native-package']
  },
  {
    id: 'mopro-flutter-lib',
    name: 'mopro flutter lib',
    description: 'Flutter package with Dart bindings for Flutter apps',
    position: { x: 190, y: 500 },
    width: 130,
    height: 50,
    color: '#FFFFFF',
    textColor: '#000000',
    category: 'mobile-native-lib',
    links: ['https://github.com/zkmopro/mopro_flutter_package']
  },

  // Mobile rust lib layer
  {
    id: 'mopro-ffi',
    name: 'mopro-ffi (mopro-cli)',
    description: 'Core Rust library using UniFFI to generate cross-platform bindings. Provides unified interface for multiple proving systems.',
    position: { x: 360, y: 400 },
    width: 160,
    height: 80,
    color: '#FFFFFF',
    textColor: '#000000',
    category: 'mobile-rust-lib',
    links: ['https://github.com/zkmopro/mopro/tree/main/mopro-ffi']
  },

  // Middleware layer
  {
    id: 'circom-compat-circom-prover',
    name: 'circom-compat circom-prover',
    displayName: ['circom-compat', 'circom-prover'],
    description: 'High-performance Rust-based Groth16 prover for Circom circuits with multiple adapter support',
    position: { x: 560, y: 320 },
    width: 160,
    height: 60,
    color: '#FFFFFF',
    textColor: '#000000',
    category: 'middleware',
    links: ['https://github.com/zkmopro/circom-compat', 'https://github.com/zkmopro/mopro/tree/main/circom-prover']
  },
  {
    id: 'halo2',
    name: 'halo2',
    description: 'Halo2 proving system implementation with support for multiple polynomial commitment schemes',
    position: { x: 560, y: 400 },
    width: 160,
    height: 50,
    color: '#FFFFFF',
    textColor: '#000000',
    category: 'middleware',
    links: ['https://github.com/privacy-scaling-explorations/halo2']
  },
  {
    id: 'noir',
    name: 'noir',
    description: 'Domain-specific language for writing zero-knowledge circuits',
    position: { x: 560, y: 470 },
    width: 160,
    height: 50,
    color: '#FFFFFF',
    textColor: '#000000',
    category: 'middleware',
    links: ['https://github.com/noir-lang/noir']
  },

  // Proving system layer
  {
    id: 'arkworks-rapidsnark',
    name: 'arkworks rapidsnark',
    displayName: ['arkworks', 'rapidsnark'],
    description: 'Fast proving backends: arkworks-rs for native Rust proving, rapidsnark for optimized C++ proving',
    position: { x: 760, y: 320 },
    width: 120,
    height: 60,
    color: '#FFFFFF',
    textColor: '#000000',
    category: 'proving-system',
    links: ['https://github.com/arkworks-rs', 'https://github.com/iden3/rapidsnark']
  },
  {
    id: 'plonk-hyperplonk-gemini',
    name: 'plonk hyperplonk gemini',
    displayName: ['plonk', 'hyperplonk', 'gemini'],
    description: 'Advanced polynomial commitment and proving schemes for Halo2',
    position: { x: 760, y: 400 },
    width: 120,
    height: 60,
    color: '#FFFFFF',
    textColor: '#000000',
    category: 'proving-system',
    links: ['https://eprint.iacr.org/2019/953', 'https://eprint.iacr.org/2022/1355', 'https://eprint.iacr.org/2022/420']
  },
  {
    id: 'barretenberg',
    name: 'barretenberg',
    description: 'High-performance proving backend developed by Aztec for Noir circuits',
    position: { x: 760, y: 480 },
    width: 120,
    height: 50,
    color: '#FFFFFF',
    textColor: '#000000',
    category: 'proving-system',
    links: ['https://github.com/AztecProtocol/barretenberg']
  },

  // Group center points (invisible components for connections)
  {
    id: 'app-center',
    name: '',
    description: '',
    position: { x: 90, y: 250 },
    width: 1,
    height: 1,
    color: 'transparent',
    category: 'connection-point'
  },
  {
    id: 'mobile-native-lib-center',
    name: '',
    description: '',
    position: { x: 255, y: 250 },
    width: 1,
    height: 1,
    color: 'transparent',
    category: 'connection-point'
  },
  {
    id: 'mobile-rust-lib-center',
    name: '',
    description: '',
    position: { x: 440, y: 250 },
    width: 1,
    height: 1,
    color: 'transparent',
    category: 'connection-point'
  },
  {
    id: 'middleware-center',
    name: '',
    description: '',
    position: { x: 640, y: 250 },
    width: 1,
    height: 1,
    color: 'transparent',
    category: 'connection-point'
  },
  {
    id: 'proving-system-center',
    name: '',
    description: '',
    position: { x: 820, y: 250 },
    width: 1,
    height: 1,
    color: 'transparent',
    category: 'connection-point'
  },
  {
    id: 'circuits-center',
    name: '',
    description: '',
    position: { x: 985, y: 250 },
    width: 1,
    height: 1,
    color: 'transparent',
    category: 'connection-point'
  },

  // Circuits layer
  {
    id: 'circom-circuits',
    name: 'circom circuits',
    description: 'Zero-knowledge circuits written in the Circom language',
    position: { x: 925, y: 320 },
    width: 120,
    height: 50,
    color: '#FFFFFF',
    textColor: '#000000',
    category: 'circuits',
    links: ['https://docs.circom.io/']
  },
  {
    id: 'halo2-circuits',
    name: 'halo2 circuits',
    description: 'Zero-knowledge circuits built with the Halo2 proving system',
    position: { x: 925, y: 400 },
    width: 120,
    height: 50,
    color: '#FFFFFF',
    textColor: '#000000',
    category: 'circuits',
    links: ['https://zcash.github.io/halo2/']
  },
  {
    id: 'noir-circuits',
    name: 'Noir circuits',
    description: 'Zero-knowledge circuits written in the Noir domain-specific language',
    position: { x: 925, y: 480 },
    width: 120,
    height: 50,
    color: '#FFFFFF',
    textColor: '#000000',
    category: 'circuits',
    links: ['https://noir-lang.org/']
  }
];

// Define connections between components
export const connections = [
  // Apps to native libs
  { from: 'ios-app', to: 'mopro-swift-lib' },
  { from: 'android-app', to: 'mopro-kotlin-lib' },
  { from: 'react-native-app', to: 'mopro-react-native-lib' },
  { from: 'flutter-app', to: 'mopro-flutter-lib' },

  // Native libs to mopro-ffi
  { from: 'mopro-swift-lib', to: 'mopro-ffi' },
  { from: 'mopro-kotlin-lib', to: 'mopro-ffi' },
  { from: 'mopro-react-native-lib', to: 'mopro-ffi' },
  { from: 'mopro-flutter-lib', to: 'mopro-ffi' },

  // Circuits to proving systems (direct connection)
  { from: 'circom-circuits', to: 'arkworks-rapidsnark' },
  { from: 'halo2-circuits', to: 'plonk-hyperplonk-gemini' },
  { from: 'noir-circuits', to: 'barretenberg' },

  // mopro-ffi to middleware
  { from: 'mopro-ffi', to: 'circom-compat-circom-prover' },
  { from: 'mopro-ffi', to: 'halo2' },
  { from: 'mopro-ffi', to: 'noir' },

  // Middleware to proving systems
  { from: 'circom-compat-circom-prover', to: 'arkworks-rapidsnark' },
  { from: 'halo2', to: 'plonk-hyperplonk-gemini' },
  { from: 'noir', to: 'barretenberg' },

  // Role connections to group centers
  { from: 'user', to: 'app-center' },
  { from: 'app-dev', to: 'mobile-native-lib-center' },
  { from: 'app-dev', to: 'mobile-rust-lib-center' },
  { from: 'tooling-infra-dev', to: 'mobile-rust-lib-center' },
  { from: 'tooling-infra-dev', to: 'middleware-center' },
  { from: 'tooling-infra-dev', to: 'proving-system-center' },
  { from: 'circuits-dev', to: 'mobile-rust-lib-center' },
  { from: 'circuits-dev', to: 'circuits-center' },
];

// Combined roles in horizontal layout
export const roleComponents = [
  {
    id: 'no-brain-label',
    name: 'No brain',
    position: { x: 90, y: 142 },
    color: '#FFCDD2',
    textColor: '#D32F2F'
  },
  {
    id: 'mobile-brain',
    name: 'Mobile brain',
    position: { x: 350, y: 142 },
    color: '#BBDEFB',
    textColor: '#1976D2'
  },
  {
    id: 'rusty-brain',
    name: 'Rusty brain',
    position: { x: 610, y: 142 },
    color: '#BBDEFB',
    textColor: '#1976D2'
  },
  {
    id: 'zk-brain',
    name: 'ZK brain',
    position: { x: 870, y: 142 },
    color: '#BBDEFB',
    textColor: '#1976D2'
  }
];

// Keep these empty for backward compatibility, remove them from React component
export const brainLabels: any[] = [];
export const devRoleLabels: any[] = [];
export const noBrainLabel = null;