import React, { createContext, useContext, useState, useEffect } from 'react';
import { webDarkTheme, webLightTheme } from '@fluentui/react-components';

const ThemeContext = createContext({
  themeMode: 'dark',
  setThemeMode: () => {},
  currentTheme: webDarkTheme,
});

export function CustomThemeProvider({ children }) {
  const [themeMode, setThemeModeState] = useState(() => {
    return localStorage.getItem('app_theme_mode') || 'dark';
  });

  const [systemIsDark, setSystemIsDark] = useState(() => {
    return window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;
  });

  useEffect(() => {
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    const handler = (e) => setSystemIsDark(e.matches);
    
    if (mediaQuery.addEventListener) {
      mediaQuery.addEventListener('change', handler);
    } else {
      mediaQuery.addListener(handler);
    }

    return () => {
      if (mediaQuery.removeEventListener) {
        mediaQuery.removeEventListener('change', handler);
      } else {
        mediaQuery.removeListener(handler);
      }
    };
  }, []);

  const setThemeMode = (mode) => {
    setThemeModeState(mode);
    localStorage.setItem('app_theme_mode', mode);
  };

  const isDark = themeMode === 'dark' || (themeMode === 'system' && systemIsDark);
  const currentTheme = isDark ? webDarkTheme : webLightTheme;

  return (
    <ThemeContext.Provider value={{ themeMode, setThemeMode, currentTheme, isDark }}>
      {children}
    </ThemeContext.Provider>
  );
}

export function useAppTheme() {
  return useContext(ThemeContext);
}
