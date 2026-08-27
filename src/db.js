import localforage from 'localforage';
import { v4 as uuidv4 } from 'uuid';

localforage.config({
  name: 'ServicePlannerDB'
});

const servicesStore = localforage.createInstance({ name: 'services' });
const itemsStore = localforage.createInstance({ name: 'items' });
const songsStore = localforage.createInstance({ name: 'songs' });
const seriesStore = localforage.createInstance({ name: 'series' });

// =======================
// SERMON SERIES
// =======================
export const getSermonSeries = async () => {
  const keys = await seriesStore.keys();
  const seriesList = await Promise.all(keys.map(k => seriesStore.getItem(k)));
  return seriesList.sort((a, b) => (a.title || '').localeCompare(b.title || ''));
};

export const getSermonSeriesById = async (id) => {
  if (!id) return null;
  return seriesStore.getItem(id);
};

export const saveSermonSeries = async (series) => {
  if (!series.id) series.id = uuidv4();
  if (!series.createdAt) series.createdAt = new Date().toISOString();
  await seriesStore.setItem(series.id, series);
  return series;
};

export const deleteSermonSeries = async (id) => {
  await seriesStore.removeItem(id);
};

// =======================
// SERVICES
// =======================
export const getServices = async () => {
  const keys = await servicesStore.keys();
  const services = await Promise.all(keys.map(k => servicesStore.getItem(k)));
  // Sort by date ascending
  return services.sort((a, b) => new Date(a.date) - new Date(b.date));
};

export const getService = async (id) => servicesStore.getItem(id);

export const saveService = async (service) => {
  if (!service.id) service.id = uuidv4();
  await servicesStore.setItem(service.id, service);
  return service;
};

export const deleteService = async (id) => {
  await servicesStore.removeItem(id);
  // Also delete associated items
  const allItems = await getItemsForService(id);
  await Promise.all(allItems.map(i => itemsStore.removeItem(i.id)));
};

export const getAllItems = async () => {
  const keys = await itemsStore.keys();
  return Promise.all(keys.map(k => itemsStore.getItem(k)));
};

export const getItemsForService = async (serviceId) => {
  const allItems = await getAllItems();
  return allItems
    .filter(item => item.serviceId === serviceId)
    .sort((a, b) => (a.order || 0) - (b.order || 0)); // Sort by order for drag and drop
};

export const saveItem = async (item) => {
  if (!item.id) item.id = uuidv4();
  await itemsStore.setItem(item.id, item);
  return item;
};

export const deleteItem = async (id) => itemsStore.removeItem(id);

// Bulk save for reordering
export const saveItems = async (items) => {
  const promises = items.map(item => itemsStore.setItem(item.id, item));
  await Promise.all(promises);
};

// =======================
// SONG BANK & ARRANGEMENTS
// =======================
// Normalizer to ensure song has arrangements array
const normalizeSong = (song) => {
  if (!song) return song;
  if (!song.arrangements || !Array.isArray(song.arrangements) || song.arrangements.length === 0) {
    const defaultArrangement = {
      id: uuidv4(),
      name: 'Default',
      key: song.defaultKey || 'C',
      bpm: song.bpm || 120,
      duration: song.defaultDuration || 300
    };
    return {
      ...song,
      arrangements: [defaultArrangement]
    };
  }
  return song;
};

export const getSongs = async () => {
  const keys = await songsStore.keys();
  const rawSongs = await Promise.all(keys.map(k => songsStore.getItem(k)));
  const normalized = rawSongs.map(normalizeSong);
  return normalized.sort((a, b) => (a.title || '').localeCompare(b.title || ''));
};

export const getSongById = async (id) => {
  if (!id) return null;
  const song = await songsStore.getItem(id);
  return normalizeSong(song);
};

export const saveSong = async (song) => {
  if (!song.id) song.id = uuidv4();
  const normalized = normalizeSong(song);
  await songsStore.setItem(normalized.id, normalized);
  return normalized;
};

export const deleteSong = async (id) => songsStore.removeItem(id);
