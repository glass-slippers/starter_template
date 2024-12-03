import { test } from '@playwright/test';
import { createHomepageTest } from './shared';

test('homepage has title and heading text and button works', createHomepageTest('http://localhost:3000'));
