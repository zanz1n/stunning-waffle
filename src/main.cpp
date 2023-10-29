#include <Arduino.h>
#include <max6675.h>
#include <LiquidCrystal_I2C.h>

enum Temperature : uint8_t
{
    T120,
    T400,
    T800,
    T1000,
    T1300,
    T1600,
    T1800,
    T2000,
};

// Macro declaration

#define PIN_HEATER_POWER 2
#define PIN_HEATER_CTRL 3
#define PIN_HEATER_INCREMENT 4
#define PIN_HEATER_DECREMENT 5
#define PIN_HEATER_RELE 7

#define PIN_TERMC_SCLK 8
#define PIN_TERMC_CS 9
#define PIN_TERMC_MISO 10

#define PIN_POWER_BTN 11
#define PIN_ON_LED 13

#define PIN_H_RELE_ON_DELAY 2000
#define PIN_H_RELE_OFF_DELAY 1000
#define PIN_H_DEFAULT_DELAY 200
#define PIN_H_CTRL_DELAY 700
#define RRB_TICK_DELAY 10
#define RRB_METRIC_INTERVAL 95

const Temperature INIT_SET_TEMPERATURE = Temperature::T2000;

// Global shared state

MAX6675 termocouple(PIN_TERMC_SCLK, PIN_TERMC_CS, PIN_TERMC_MISO);
LiquidCrystal_I2C lcd(0x27, 16, 2);

Temperature current_temp = Temperature::T1300;
unsigned int rrb_state_count = 1;
bool turned_on = false;

void setup()
{
    pinMode(PIN_HEATER_POWER, OUTPUT);
    pinMode(PIN_HEATER_CTRL, OUTPUT);
    pinMode(PIN_HEATER_INCREMENT, OUTPUT);
    pinMode(PIN_HEATER_DECREMENT, OUTPUT);

    pinMode(PIN_HEATER_RELE, OUTPUT);

    pinMode(PIN_POWER_BTN, INPUT);
    pinMode(PIN_ON_LED, OUTPUT);

    Serial.begin(9600);

    lcd.init();
    lcd.setBacklight(0x1);
}

inline void turn_heater_power()
{
    digitalWrite(PIN_HEATER_POWER, HIGH);
    delay(PIN_H_DEFAULT_DELAY);
    digitalWrite(PIN_HEATER_POWER, LOW);
}

inline void turn_heater_ctrl()
{
    digitalWrite(PIN_HEATER_CTRL, HIGH);
    delay(PIN_H_CTRL_DELAY);
    digitalWrite(PIN_HEATER_CTRL, LOW);
}

inline void turn_heater_increment()
{
    digitalWrite(PIN_HEATER_INCREMENT, HIGH);
    delay(PIN_H_DEFAULT_DELAY);
    digitalWrite(PIN_HEATER_INCREMENT, LOW);
}

inline void turn_heater_decrement()
{
    digitalWrite(PIN_HEATER_DECREMENT, HIGH);
    delay(PIN_H_DEFAULT_DELAY);
    digitalWrite(PIN_HEATER_DECREMENT, LOW);
}

char *temperature_display_watts(Temperature t)
{
    switch (t)
    {
    case Temperature::T120:
        return " 120w";
    case Temperature::T400:
        return " 400w";
    case Temperature::T800:
        return "800w";
    case Temperature::T1000:
        return "1000w";
    case Temperature::T1300:
        return "1300w";
    case Temperature::T1600:
        return "1600w";
    case Temperature::T1800:
        return "1800w";
    case Temperature::T2000:
        return "2000w";
    }
}

void set_temperature(const Temperature temp)
{
    if (temp > current_temp)
    {
        for (int8_t dif = temp - current_temp; dif > 0; dif--)
        {
            delay(PIN_H_DEFAULT_DELAY);
            turn_heater_increment();
        }
    }
    else if (current_temp > temp)
    {
        for (int8_t dif = current_temp - temp; dif > 0; dif--)
        {
            delay(PIN_H_DEFAULT_DELAY);
            turn_heater_decrement();
        }
    }

    current_temp = temp;
}

inline void adjust_heater()
{
    current_temp = Temperature::T1300;
    turn_heater_ctrl();
    set_temperature(INIT_SET_TEMPERATURE);
}

void submit_metrics()
{
    float celcius = termocouple.readCelsius();

    lcd.clear();
    lcd.setCursor(0, 0);

    if (turned_on)
        lcd.print("Heater ON  ");
    else
        lcd.print("Heater OFF ");

    lcd.print(temperature_display_watts(current_temp));

    lcd.setCursor(0, 1);
    lcd.print(String("H: ") + String(celcius, 1));
    lcd.print("  ");
    lcd.print(String("S: ") + String(celcius, 1));

    String json_str = String("{\"temperature_1\":" + String(celcius, DEC) + "}");

    Serial.println(json_str.c_str());
}

void change_power_state()
{
    if (!turned_on)
    {
        digitalWrite(PIN_HEATER_RELE, 0x1);
        delay(PIN_H_RELE_ON_DELAY);

        turn_heater_power();
        adjust_heater();
    }
    else
    {
        turn_heater_power();

        delay(PIN_H_RELE_OFF_DELAY);
        digitalWrite(PIN_HEATER_RELE, 0x0);

        current_temp = Temperature::T1300;
    }

    turned_on = !turned_on;
    digitalWrite(PIN_ON_LED, turned_on);
}

void loop()
{
    if (rrb_state_count > RRB_METRIC_INTERVAL)
    {
        rrb_state_count = 1;
        submit_metrics();
    }
    else if (digitalRead(PIN_POWER_BTN))
    {
        change_power_state();
        rrb_state_count = RRB_METRIC_INTERVAL;
    }
    rrb_state_count++;

    delay(RRB_TICK_DELAY);
}