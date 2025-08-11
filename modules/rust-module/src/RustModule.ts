import { NativeModule, requireNativeModule } from 'expo';

import { RustModuleEvents } from './RustModule.types';

declare class RustModule extends NativeModule<RustModuleEvents> {
  PI: number;
  hello(): string;
  setValueAsync(value: string): Promise<void>;
}

// This call loads the native module object from the JSI.
export default requireNativeModule<RustModule>('RustModule');
