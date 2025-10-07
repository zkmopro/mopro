package com.example.moproapp

import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.junit4.createComposeRule
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.performClick
import androidx.test.platform.app.InstrumentationRegistry
import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.Test
import org.junit.runner.RunWith
import org.junit.Before
import org.junit.After
import org.junit.Assert.*
import org.junit.Rule

/**
 * Instrumented test, which will execute on an Android device.
 *
 * See [testing documentation](http://d.android.com/tools/testing).
 */
@RunWith(AndroidJUnit4::class)
class ExampleInstrumentedTest {
    @get:Rule
    val composeTestRule = createComposeRule()
    
    @Before
    fun setUp() {
        Thread.sleep(1000) // Wait for previous test cleanup
    }
    
    @After 
    fun tearDown() {
        Thread.sleep(2000) // Wait for background threads
    }
    
    private fun waitForProofCompletion(verifyButtonTag: String, maxWaitSeconds: Int = 35) {
        composeTestRule.waitUntil(timeoutMillis = maxWaitSeconds * 1000L) {
            try {
                composeTestRule.onNodeWithTag(verifyButtonTag).assertExists()
                true
            } catch (e: Exception) {
                false
            }
        }
    }

    @Test
    fun useAppContext() {
        val appContext = InstrumentationRegistry.getInstrumentation().targetContext
        assertEquals("com.example.moproapp", appContext.packageName)
    }

    @Test
    fun circomButtonClick() {
        composeTestRule.setContent { MultiplierComponent() }
        
        composeTestRule.onNodeWithTag("circomGenerateProofButton").performClick()
        composeTestRule.onNodeWithTag("circomGenerateProofButton").assertIsDisplayed()
        
        waitForProofCompletion("circomVerifyProofButton", maxWaitSeconds = 10)
        
        composeTestRule.onNodeWithTag("circomVerifyProofButton").performClick()
        composeTestRule.onNodeWithTag("circomVerifyProofButton").assertIsDisplayed()
    }

    @Test 
    fun noirButtonClick() {
        composeTestRule.setContent { NoirComponent() }

        composeTestRule.onNodeWithTag("noirGenerateProofButton").performClick()
        composeTestRule.onNodeWithTag("noirGenerateProofButton").assertIsDisplayed()

        waitForProofCompletion("noirVerifyProofButton", maxWaitSeconds = 10)

        composeTestRule.onNodeWithTag("noirVerifyProofButton").performClick()
        composeTestRule.onNodeWithTag("noirVerifyProofButton").assertIsDisplayed()
    }

    @Test
    fun rapidsnarkButtonClick() {
        composeTestRule.setContent { MultiplierComponent() }

        composeTestRule.onNodeWithTag("rapidsnarkGenerateProofButton").performClick()
        composeTestRule.onNodeWithTag("rapidsnarkGenerateProofButton").assertIsDisplayed()

        waitForProofCompletion("rapidsnarkVerifyProofButton", maxWaitSeconds = 10)

        composeTestRule.onNodeWithTag("rapidsnarkVerifyProofButton").performClick()
        composeTestRule.onNodeWithTag("rapidsnarkVerifyProofButton").assertIsDisplayed()
    }

    @Test
    fun halo2ButtonClick() {
        composeTestRule.setContent { FibonacciComponent() }

        composeTestRule.onNodeWithTag("halo2GenerateProofButton").performClick()
        composeTestRule.onNodeWithTag("halo2GenerateProofButton").assertIsDisplayed()

        waitForProofCompletion("halo2VerifyProofButton", maxWaitSeconds = 10)

        composeTestRule.onNodeWithTag("halo2VerifyProofButton").performClick()
        composeTestRule.onNodeWithTag("halo2VerifyProofButton").assertIsDisplayed()
    }
}