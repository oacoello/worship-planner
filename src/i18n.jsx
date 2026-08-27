import React, { createContext, useContext, useState, useEffect } from 'react';

const translations = {
  en: {
    // Sidebar & Navigation
    appTitle: 'Granco Planner',
    services: 'Services',
    calendar: 'Calendar',
    series: 'Series',
    songBank: 'Song Bank',
    stats: 'Analytics',
    settings: 'Settings',

    // Calendar View
    calendarTitle: 'Calendar',
    calendarDesc: 'View and manage all services and meetings on a monthly schedule',
    today: 'Today',
    sun: 'Sun',
    mon: 'Mon',
    tue: 'Tue',
    wed: 'Wed',
    thu: 'Thu',
    fri: 'Fri',
    sat: 'Sat',
    addServiceForDay: 'Add Service for this date',

    // Statistics & Analytics
    analyticsTitle: 'Song Analytics & Metrics',
    analyticsDesc: 'Frequency, rotation, and usage statistics of songs across worship services',
    totalServicesScheduled: 'Total Services',
    totalSongInstances: 'Songs Scheduled',
    uniqueSongsUsed: 'Unique Songs in Rotation',
    mostFrequentSong: 'Top Song',
    mostFrequentKey: 'Most Used Key',
    mostPlayedTitle: 'Most Frequently Scheduled Songs',
    leastPlayedTitle: 'Least Scheduled / Inactive Songs',
    timesPlayed: 'times played',
    lastPlayedDate: 'Last scheduled',
    neverPlayedLabel: 'Never scheduled',
    songTitleCol: 'Song',
    artistCol: 'Artist / Author',
    usageCountCol: 'Count',
    frequencyCol: 'Frequency',
    lastDateCol: 'Last Used',

    // Services List
    servicePlans: 'Service Plans',
    servicePlansDesc: 'Plan worship setlists, sermon topics, and technical run sheets',
    newService: 'New Service',
    editServicePlan: 'Edit Service Plan',
    createNewService: 'Create New Service',
    serviceName: 'Service Name',
    sermonSeries: 'Sermon Series',
    createNewSeries: 'Create New Series',
    standaloneNoSeries: '-- Standalone / No Series --',
    cancelPickExisting: 'Cancel / Pick Existing',
    sermonTitleTopic: 'Sermon Title / Message Topic',
    weekPart: 'Week / Part #',
    date: 'Date',
    time: 'Time',
    createService: 'Create Service',
    saveChanges: 'Save Changes',
    cancel: 'Cancel',
    editPlan: 'Edit Plan',
    delete: 'Delete',
    noServicesFound: 'No services found. Click "New Service" to start planning!',
    confirmDeleteService: 'Are you sure you want to delete this service plan?',
    standaloneWorshipService: 'Standalone Worship Service',
    standaloneServicesGroup: 'Standalone Services',

    // Sermon Series
    sermonSeriesTitle: 'Sermon Series',
    sermonSeriesDesc: 'Manage sermon graphic artwork and themes for your services',
    newSeries: 'New Series',
    editSeries: 'Edit Sermon Series',
    createSermonSeries: 'Create Sermon Series',
    seriesTitle: 'Series Title',
    descriptionOptional: 'Description (Optional)',
    seriesArtworkGraphic: 'Series Artwork / Graphic',
    dropzoneText: 'Click or drag & drop series graphic / slide art here',
    noArtworkUploaded: 'No Artwork Uploaded',
    saveSeries: 'Save Series',
    noSeriesFound: 'No sermon series found. Click "New Series" to create one with banner graphics!',
    confirmDeleteSeries: 'Are you sure you want to delete this sermon series? Services linked to it will retain their title.',

    // Service Builder / Run Sheet
    backToServices: 'Back to Services',
    editPlanDetails: 'Edit Plan Details',
    printRunSheet: 'Print Run Sheet',
    savePngScreenshot: 'Save PNG Screenshot',
    addSong: 'Add Song',
    addVideo: 'Add Video',
    addLoop: 'Add Loop',
    addAnnouncements: 'Add Announcements',
    addConference: 'Add Conference',
    addCountdown: 'Add Countdown',
    start: 'Start',
    itemArrangement: 'Item & Arrangement',
    notesCues: 'Notes & Cues',
    leadVocalist: 'Lead / Vocalist',
    length: 'Length',
    chooseFromSongBank: 'Choose from Master Song Bank',
    customManualEntry: '-- Custom / Manual Entry --',
    arrangement: 'Arrangement',
    titleName: 'Title / Name',
    arrangementName: 'Arrangement Name',
    key: 'Key',
    mainVocalistLead: 'Main Vocalist / Lead Singer',
    speaker: 'Speaker / Preacher',
    durationMinutes: 'Duration (Minutes)',
    durationSeconds: 'Duration (Seconds)',
    extraNotesTechnicalCues: 'Extra Notes & Technical Cues',
    addToSheet: 'Add to Sheet',
    saveNotes: 'Save Notes',
    emptySheetPrompt: 'Your service sheet is empty. Use the buttons in the toolbar above to add items.',
    total: 'Total',
    estEnd: 'Est. End',
    seriesPrefix: 'Series',

    // Song Bank
    songBankTitle: 'Master Song Bank',
    songBankDesc: 'Manage your song library, multi-arrangements, musical keys, and tempos',
    newSong: 'New Song',
    searchSongs: 'Search by song title or artist...',
    arrangements: 'Arrangements',
    addArrangement: 'Add Arrangement',
    songTitle: 'Song Title',
    artistAuthor: 'Artist / Author',
    defaultArrangementName: 'Default Arrangement Name',
    bpm: 'BPM',
    change: 'Change',
    noResultsFound: 'No matching songs found',
    addSongToBank: 'Add Song to Bank',
    noSongsFound: 'No songs found. Add songs to your bank to quickly insert them into services!',
    confirmDeleteSong: 'Are you sure you want to delete this song and all its arrangements?',

    // Settings
    settingsTitle: 'Settings',
    settingsSubtitle: 'Configure application theme, language, and preferences',
    appearance: 'Appearance & Theme',
    themeDescription: 'Choose how Granco Planner looks on your device',
    darkTheme: 'Dark Mode',
    lightTheme: 'Light Mode',
    systemTheme: 'System (Match OS)',
    language: 'Language',
    languageDescription: 'Select your preferred user interface language',
    english: 'English',
    spanish: 'Español'
  },
  es: {
    // Sidebar & Navigation
    appTitle: 'Granco Planner',
    services: 'Servicios',
    calendar: 'Calendar',
    series: 'Series',
    songBank: 'Cancionero',
    stats: 'Estadísticas',
    settings: 'Configuración',

    // Calendar View
    calendarTitle: 'Calendar',
    calendarDesc: 'Visualiza y gestiona todos tus servicios y reuniones en un calendario mensual',
    today: 'Hoy',
    sun: 'Dom',
    mon: 'Lun',
    tue: 'Mar',
    wed: 'Mié',
    thu: 'Jue',
    fri: 'Vie',
    sat: 'Sáb',
    addServiceForDay: 'Agregar servicio en esta fecha',

    // Statistics & Analytics
    analyticsTitle: 'Estadísticas y Rotación de Canciones',
    analyticsDesc: 'Métricas de frecuencia, conteo de uso y rotación de alabanzas en las reuniones',
    totalServicesScheduled: 'Total de Reuniones',
    totalSongInstances: 'Canciones Programadas',
    uniqueSongsUsed: 'Canciones Únicas Usadas',
    mostFrequentSong: 'Canción Más Cantada',
    mostFrequentKey: 'Tono Más Frecuente',
    mostPlayedTitle: 'Canciones Más Frecuentes',
    leastPlayedTitle: 'Canciones Menos Usadas / Sin Rotación Reciente',
    timesPlayed: 'veces cantada',
    lastPlayedDate: 'Última vez',
    neverPlayedLabel: 'Nunca programada',
    songTitleCol: 'Canción',
    artistCol: 'Artista / Autor',
    usageCountCol: 'Veces',
    frequencyCol: 'Frecuencia',
    lastDateCol: 'Última Fecha',

    // Services List
    servicePlans: 'Planes de Servicio',
    servicePlansDesc: 'Planifica repertorios de alabanza, temas de sermón y órdenes de culto',
    newService: 'Nuevo Servicio',
    editServicePlan: 'Editar Plan de Servicio',
    createNewService: 'Crear Nuevo Servicio',
    serviceName: 'Nombre del Servicio',
    sermonSeries: 'Serie de Sermones',
    createNewSeries: 'Crear Nueva Serie',
    standaloneNoSeries: '-- Servicio Independiente / Sin Serie --',
    cancelPickExisting: 'Cancelar / Elegir Existente',
    sermonTitleTopic: 'Título del Sermón / Tema',
    weekPart: 'Semana / Parte #',
    date: 'Fecha',
    time: 'Hora',
    createService: 'Crear Servicio',
    saveChanges: 'Guardar Cambios',
    cancel: 'Cancelar',
    editPlan: 'Editar Plan',
    delete: 'Eliminar',
    noServicesFound: 'No hay servicios planificados. ¡Haz clic en "Nuevo Servicio" para comenzar!',
    confirmDeleteService: '¿Estás seguro de que deseas eliminar este plan de servicio?',
    standaloneWorshipService: 'Servicio de Alabanza Independiente',
    standaloneServicesGroup: 'Servicios Independientes',

    // Sermon Series
    sermonSeriesTitle: 'Series de Sermones',
    sermonSeriesDesc: 'Administra gráficas, portadas y temas para tus servicios',
    newSeries: 'Nueva Serie',
    editSeries: 'Editar Serie de Sermones',
    createSermonSeries: 'Crear Serie de Sermones',
    seriesTitle: 'Título de la Serie',
    descriptionOptional: 'Descripción (Opcional)',
    seriesArtworkGraphic: 'Gráfica / Portada de la Serie',
    dropzoneText: 'Haz clic o arrastra y suelta la imagen aquí',
    noArtworkUploaded: 'Sin portada cargada',
    saveSeries: 'Guardar Serie',
    noSeriesFound: 'No hay series creadas. ¡Haz clic en "Nueva Serie" para agregar una!',
    confirmDeleteSeries: '¿Estás seguro de que deseas eliminar esta serie? Los servicios asociados conservarán su nombre.',

    // Service Builder / Run Sheet
    backToServices: 'Volver a Servicios',
    editPlanDetails: 'Editar Detalles del Plan',
    printRunSheet: 'Imprimir Orden de Culto',
    savePngScreenshot: 'Guardar Captura PNG',
    addSong: 'Agregar Canción',
    addVideo: 'Agregar Video',
    addLoop: 'Agregar Loop',
    addAnnouncements: 'Agregar Anuncios',
    addConference: 'Agregar Conferencia',
    addCountdown: 'Agregar Cuenta Regresiva',
    start: 'Inicio',
    itemArrangement: 'Elemento y Arreglo',
    notesCues: 'Notas y Cues',
    leadVocalist: 'Voz Principal / Líder',
    length: 'Duración',
    chooseFromSongBank: 'Elegir del Cancionero',
    customManualEntry: '-- Entrada Manual / Personalizada --',
    arrangement: 'Arreglo',
    titleName: 'Título / Nombre',
    arrangementName: 'Nombre del Arreglo',
    key: 'Tono',
    mainVocalistLead: 'Vocalista Principal / Líder de Alabanza',
    speaker: 'Conferencista / Predicador',
    durationMinutes: 'Duración (Minutos)',
    durationSeconds: 'Duración (Segundos)',
    extraNotesTechnicalCues: 'Notas Adicionales y Cues Técnicos',
    addToSheet: 'Agregar al Plan',
    saveNotes: 'Guardar Notas',
    emptySheetPrompt: 'La hoja de servicio está vacía. Usa la barra superior para agregar elementos.',
    total: 'Total',
    estEnd: 'Fin Est.',
    seriesPrefix: 'Serie',

    // Song Bank
    songBankTitle: 'Cancionero General',
    songBankDesc: 'Administra tu biblioteca de canciones, arreglos, tonos y tempos',
    newSong: 'Nueva Canción',
    searchSongs: 'Buscar por título o artista...',
    arrangements: 'Arreglos',
    addArrangement: 'Agregar Arreglo',
    songTitle: 'Título de la Canción',
    artistAuthor: 'Artista / Autor',
    defaultArrangementName: 'Nombre del Arreglo Predeterminado',
    bpm: 'BPM',
    change: 'Cambiar',
    noResultsFound: 'No se encontraron canciones con ese nombre',
    addSongToBank: 'Guardar Canción',
    noSongsFound: 'No hay canciones en el cancionero. ¡Agrega tus temas para insertarlos rápidamente!',
    confirmDeleteSong: '¿Estás seguro de que deseas eliminar esta canción y todos sus arreglos?',

    // Settings
    settingsTitle: 'Configuración',
    settingsSubtitle: 'Personaliza el tema visual, el idioma y las preferencias',
    appearance: 'Apariencia y Tema',
    themeDescription: 'Elige cómo se ve Granco Planner en tu dispositivo',
    darkTheme: 'Modo Oscuro',
    lightTheme: 'Modo Claro',
    systemTheme: 'Sistema (Predeterminado del SO)',
    language: 'Idioma',
    languageDescription: 'Selecciona el idioma de la interfaz',
    english: 'English (Inglés)',
    spanish: 'Español'
  }
};

const I18nContext = createContext({
  language: 'es',
  setLanguage: () => {},
  t: (key) => key
});

export function I18nProvider({ children }) {
  const [language, setLanguageState] = useState(() => {
    return localStorage.getItem('app_language') || 'es';
  });

  const setLanguage = (lang) => {
    setLanguageState(lang);
    localStorage.setItem('app_language', lang);
  };

  const t = (key) => {
    return translations[language]?.[key] || translations['en']?.[key] || key;
  };

  return (
    <I18nContext.Provider value={{ language, setLanguage, t }}>
      {children}
    </I18nContext.Provider>
  );
}

export function useI18n() {
  return useContext(I18nContext);
}
