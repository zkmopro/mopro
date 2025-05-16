package com.mopro.mopro_app

import MultiplierComponent
import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.assertIsEnabled
import androidx.compose.ui.test.junit4.createComposeRule
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.performClick
import androidx.test.platform.app.InstrumentationRegistry
import androidx.test.ext.junit.runners.AndroidJUnit4

import org.junit.Test
import org.junit.runner.RunWith

import org.junit.Assert.*
import org.junit.Rule

/**
 * Instrumented test, which will execute on an Android device.
 *
 * See [testing documentation](http://d.android.com/tools/testing).
 */
@RunWith(AndroidJUnit4::class)
class ExampleInstrumentedTest {
    @Test
    fun useAppContext() {
        // Context of the app under test.
        val appContext = InstrumentationRegistry.getInstrumentation().targetContext
        assertEquals("com.mopro.mopro_app", appContext.packageName)
    }

    @get:Rule
    val composeTestRule = createComposeRule()

    @Test
    fun testCircomButtonClick() {
        // Set up the Compose UI
        composeTestRule.setContent {
            MultiplierComponent() // Replace with the actual Composable function
        }

        // Test click circom generate proof button
        composeTestRule.onNodeWithTag("circomGenerateProofButton").performClick()
        composeTestRule.onNodeWithTag("circomGenerateProofButton").assertIsDisplayed()

        // Wait up to 5 seconds for any circomVerifyProofButton to become enabled before continuing the test
        composeTestRule.waitUntil(timeoutMillis = 5000) {
            try {
                composeTestRule.onNodeWithTag("circomVerifyProofButton").assertIsEnabled()
                true
            } catch (e: Exception) {
                false
            }
        }
        composeTestRule.onNodeWithTag("circomVerifyProofButton").assertIsDisplayed()
    }

    @Test
    fun testHalo2ButtonClick() {
        // Set up the Compose UI
        composeTestRule.setContent {
            FibonacciComponent() // Replace with the actual Composable function
        }

        // Test click circom generate proof button
        composeTestRule.onNodeWithTag("halo2GenerateProofButton").performClick()
        composeTestRule.onNodeWithTag("halo2GenerateProofButton").assertIsDisplayed()

        // Wait up to 5 seconds for any halo2VerifyProofButton to become enabled before continuing the test
        composeTestRule.waitUntil(timeoutMillis = 5000) {
            try {
                composeTestRule.onNodeWithTag("halo2VerifyProofButton").assertIsEnabled()
                true
            } catch (e: Exception) {
                false
            }
        }

        composeTestRule.onNodeWithTag("halo2VerifyProofButton").assertIsDisplayed()
    }
}